//! styx-vfs-layer — 9P2000.L Bookshelf daemon (GB-03 / ATOM-GB03-STYX-20260713)
//!
//! Served roots (read): `.atom-trail/`, `docs/schemas/v0.1/`,
//! `notebooks/triweave_backend_results/`, `verification_certificates/` (optional).
//! Write root: `.atom-trail/decisions/` ONLY.
//! Transport v0: TCP 127.0.0.1:5640 (AF_VSOCK stretch; not required).
//!
//! Design note: WSL2's Windows↔Linux interop is itself Plan 9 — this promotes
//! the host-native protocol to first-class for the Bookshelf, it does not import
//! a foreign one.

use anyhow::{anyhow, bail, Context, Result};
use bytes::{Buf, BufMut, BytesMut};
use chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{Read as _, Seek, SeekFrom, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{info, warn};

pub const DEFAULT_ADDR: &str = "127.0.0.1:5640";
pub const IOUNIT: u32 = 8192;

// 9P2000 message types
const TVERSION: u8 = 100;
const RVERSION: u8 = 101;
const TAUTH: u8 = 102;
const RERROR: u8 = 107;
const TFLUSH: u8 = 108;
const RFLUSH: u8 = 109;
const TATTACH: u8 = 104;
const RATTACH: u8 = 105;
const TWALK: u8 = 110;
const RWALK: u8 = 111;
const TOPEN: u8 = 112;
const ROPEN: u8 = 113;
const TCREATE: u8 = 114;
const TREAD: u8 = 116;
const RREAD: u8 = 117;
const TWRITE: u8 = 118;
const RWRITE: u8 = 119;
const TCLUNK: u8 = 120;
const RCLUNK: u8 = 121;
const TREMOVE: u8 = 122;
const TSTAT: u8 = 124;
const RSTAT: u8 = 125;
const TWSTAT: u8 = 126;

// 9P2000.L
const TLOPEN: u8 = 12;
const RLOPEN: u8 = 13;
const TLCREATE: u8 = 14;
const TGETATTR: u8 = 24;
const RGETATTR: u8 = 25;
const TSETATTR: u8 = 26;
const TREADDIR: u8 = 40;
const RREADDIR: u8 = 41;
const TSTATFS: u8 = 8;
const RSTATFS: u8 = 9;

const QTDIR: u8 = 0x80;
const QTFILE: u8 = 0x00;

const NOTAG: u16 = 0xFFFF;
const NOFID: u32 = 0xFFFFFFFF;

#[derive(Clone)]
pub struct BookshelfConfig {
    pub repo_root: PathBuf,
    pub listen: String,
}

impl BookshelfConfig {
    pub fn from_env() -> Self {
        let repo_root = std::env::var("LOGOS_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        let listen = std::env::var("STYX_LISTEN").unwrap_or_else(|_| DEFAULT_ADDR.into());
        Self { repo_root, listen }
    }
}

/// Top-level names exposed at the 9P root (read-only except decisions under atom-trail).
fn root_entries(repo: &Path) -> Vec<(String, PathBuf, bool)> {
    let mut v = vec![
        (
            "atom-trail".into(),
            repo.join(".atom-trail"),
            false, // not directly writable at dir; decisions is
        ),
        (
            "schemas".into(),
            repo.join("docs/schemas/v0.1"),
            false,
        ),
        (
            "notebooks".into(),
            repo.join("notebooks/triweave_backend_results"),
            false,
        ),
    ];
    let certs = repo.join("verification_certificates");
    if certs.is_dir() {
        v.push(("verification_certificates".into(), certs, false));
    }
    v
}

fn is_write_allowed(repo: &Path, real: &Path) -> bool {
    let decisions = match repo.join(".atom-trail/decisions").canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };
    let real = match real.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // file may not exist yet — check parent
            if let Some(parent) = real.parent() {
                if let Ok(p) = parent.canonicalize() {
                    return p == decisions || p.starts_with(&decisions);
                }
            }
            return false;
        }
    };
    real == decisions || real.starts_with(&decisions)
}

fn log_write(path: &str, size: usize, coherence: f64) {
    info!(
        "ATOM: 9P-WRITE | path={} | size={} | coherence={:.4}",
        path, size, coherence
    );
}

fn emit_void_event(repo: &Path, path: &str, reason: &str) -> Result<()> {
    let trail = repo.join(".atom-trail");
    fs::create_dir_all(&trail)?;
    let ts = Utc::now().format("%Y%m%dT%H%M%SZ");
    let name = format!("VOID-9P-{ts}.json");
    let file = trail.join(&name);
    let body = json!({
        "kind": "VOID",
        "source": "styx-vfs-layer",
        "atom": "ATOM-GB03-STYX-20260713",
        "path": path,
        "reason": reason,
        "timestamp": Utc::now().to_rfc3339(),
        "invariant": "alpha + omega = 15",
    });
    fs::write(&file, serde_json::to_vec_pretty(&body)?)?;
    info!("VOID event: {}", file.display());
    Ok(())
}

#[derive(Clone)]
struct Qid {
    qtype: u8,
    version: u32,
    path: u64,
}

impl Qid {
    fn for_path(p: &Path, path_id: u64) -> Self {
        let qtype = if p.is_dir() { QTDIR } else { QTFILE };
        let version = p
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);
        Self {
            qtype,
            version,
            path: path_id,
        }
    }

    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u8(self.qtype);
        buf.put_u32_le(self.version);
        buf.put_u64_le(self.path);
    }
}

struct Fid {
    real: PathBuf,
    /// logical path from 9P root, e.g. "schemas/certificate.schema.json"
    logical: String,
    omode: Option<u8>,
    file: Option<fs::File>,
}

struct Session {
    fids: HashMap<u32, Fid>,
    path_ids: HashMap<PathBuf, u64>,
    next_path_id: AtomicU64,
    msize: u32,
    repo: PathBuf,
}

impl Session {
    fn new(repo: PathBuf) -> Self {
        Self {
            fids: HashMap::new(),
            path_ids: HashMap::new(),
            next_path_id: AtomicU64::new(1),
            msize: 8192,
            repo,
        }
    }

    fn path_id(&mut self, p: &Path) -> u64 {
        if let Some(id) = self.path_ids.get(p) {
            return *id;
        }
        let id = self.next_path_id.fetch_add(1, Ordering::Relaxed);
        self.path_ids.insert(p.to_path_buf(), id);
        id
    }

    fn qid_of(&mut self, p: &Path) -> Qid {
        let id = self.path_id(p);
        Qid::for_path(p, id)
    }
}

fn put_string(buf: &mut BytesMut, s: &str) {
    buf.put_u16_le(s.len() as u16);
    buf.put_slice(s.as_bytes());
}

fn get_string(buf: &mut impl Buf) -> Result<String> {
    if buf.remaining() < 2 {
        bail!("short string len");
    }
    let n = buf.get_u16_le() as usize;
    if buf.remaining() < n {
        bail!("short string body");
    }
    let mut v = vec![0u8; n];
    buf.copy_to_slice(&mut v);
    Ok(String::from_utf8_lossy(&v).into_owned())
}

fn rerror(tag: u16, msg: &str) -> BytesMut {
    let mut body = BytesMut::new();
    body.put_u8(RERROR);
    body.put_u16_le(tag);
    put_string(&mut body, msg);
    frame(body)
}

fn frame(body: BytesMut) -> BytesMut {
    let mut out = BytesMut::with_capacity(4 + body.len());
    out.put_u32_le((4 + body.len()) as u32);
    out.extend_from_slice(&body);
    out
}

fn handle(session: &mut Session, msg: &[u8]) -> Result<BytesMut> {
    if msg.len() < 7 {
        bail!("message too short");
    }
    let mut cur = &msg[4..]; // skip size
    let mtype = cur.get_u8();
    let tag = cur.get_u16_le();

    match mtype {
        TVERSION => {
            let msize = cur.get_u32_le();
            let version = get_string(&mut cur)?;
            session.msize = msize.min(64 * 1024);
            let ver = if version.starts_with("9P2000") {
                "9P2000.L"
            } else {
                "9P2000.L"
            };
            let mut body = BytesMut::new();
            body.put_u8(RVERSION);
            body.put_u16_le(tag);
            body.put_u32_le(session.msize);
            put_string(&mut body, ver);
            Ok(frame(body))
        }
        TAUTH => Ok(rerror(tag, "authentication not required")),
        TFLUSH => {
            let mut body = BytesMut::new();
            body.put_u8(RFLUSH);
            body.put_u16_le(tag);
            Ok(frame(body))
        }
        TATTACH => {
            let fid = cur.get_u32_le();
            let _afid = cur.get_u32_le();
            let _uname = get_string(&mut cur)?;
            let _aname = get_string(&mut cur)?;
            // root is virtual
            let root = session.repo.clone();
            let qid = session.qid_of(&root);
            // synthetic root — use repo path as anchor but readdir is virtual
            session.fids.insert(
                fid,
                Fid {
                    real: root,
                    logical: String::new(),
                    omode: None,
                    file: None,
                },
            );
            let mut body = BytesMut::new();
            body.put_u8(RATTACH);
            body.put_u16_le(tag);
            qid.encode(&mut body);
            Ok(frame(body))
        }
        TWALK => {
            let fid = cur.get_u32_le();
            let newfid = cur.get_u32_le();
            let nwname = cur.get_u16_le() as usize;
            let mut names = Vec::with_capacity(nwname);
            for _ in 0..nwname {
                names.push(get_string(&mut cur)?);
            }
            let base = session
                .fids
                .get(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?
                .clone();
            let mut logical = base.logical.clone();
            let mut real = base.real.clone();
            let mut qids = Vec::new();

            if names.is_empty() {
                // clone fid
                let qid = session.qid_of(&real);
                session.fids.insert(newfid, base);
                let mut body = BytesMut::new();
                body.put_u8(RWALK);
                body.put_u16_le(tag);
                body.put_u16_le(0);
                let _ = qid;
                return Ok(frame(body));
            }

            for (i, name) in names.iter().enumerate() {
                if name == ".." {
                    if logical.is_empty() {
                        // stay at root
                    } else if let Some(idx) = logical.rfind('/') {
                        logical.truncate(idx);
                        real = resolve_logical(&session.repo, &logical)?;
                    } else {
                        logical.clear();
                        real = session.repo.clone();
                    }
                } else if name != "." {
                    if logical.is_empty() {
                        logical = name.clone();
                    } else {
                        logical = format!("{logical}/{name}");
                    }
                    real = match resolve_logical(&session.repo, &logical) {
                        Ok(p) => p,
                        Err(e) => {
                            if i == 0 {
                                return Ok(rerror(tag, &format!("file not found: {e}")));
                            }
                            break;
                        }
                    };
                    if !real.exists() && i + 1 < names.len() {
                        return Ok(rerror(tag, "file not found"));
                    }
                }
                if real.exists() || logical.is_empty() {
                    let qpath = if logical.is_empty() {
                        session.repo.clone()
                    } else {
                        real.clone()
                    };
                    let q = session.qid_of(&qpath);
                    qids.push(q);
                } else {
                    break;
                }
            }

            if qids.len() != names.len() && qids.is_empty() {
                return Ok(rerror(tag, "file not found"));
            }

            if qids.len() == names.len() {
                session.fids.insert(
                    newfid,
                    Fid {
                        real: if logical.is_empty() {
                            session.repo.clone()
                        } else {
                            real
                        },
                        logical,
                        omode: None,
                        file: None,
                    },
                );
            }

            let mut body = BytesMut::new();
            body.put_u8(RWALK);
            body.put_u16_le(tag);
            body.put_u16_le(qids.len() as u16);
            for q in qids {
                q.encode(&mut body);
            }
            Ok(frame(body))
        }
        TOPEN | TLOPEN => {
            let fid = cur.get_u32_le();
            let mode = if mtype == TLOPEN {
                // Tlopen: flags u32
                cur.get_u32_le() as u8
            } else {
                cur.get_u8()
            };
            let want_write = mode & 0x01 != 0 || mode & 0x02 != 0; // OWRITE/ORDWR (classic) approx
            let l_write = mode & 0x1 != 0 || mode & 0x2 != 0; // O_WRONLY/O_RDWR
            let writing = want_write || l_write || (mode & 0x41) != 0;

            let (real, logical) = {
                let fid_ent = session
                    .fids
                    .get(&fid)
                    .ok_or_else(|| anyhow!("unknown fid"))?;
                (fid_ent.real.clone(), fid_ent.logical.clone())
            };

            if writing && !is_write_allowed(&session.repo, &real) {
                let _ = emit_void_event(
                    &session.repo,
                    &logical,
                    "write outside .atom-trail/decisions/",
                );
                return Ok(rerror(tag, "permission denied: write root is .atom-trail/decisions only"));
            }

            let qid = session.qid_of(&real);
            let fid_ent = session
                .fids
                .get_mut(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?;
            if real.is_file() {
                let f = if writing {
                    fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&real)
                        .with_context(|| format!("open {}", real.display()))?
                } else {
                    fs::File::open(&real)
                        .with_context(|| format!("open {}", real.display()))?
                };
                fid_ent.file = Some(f);
            }
            fid_ent.omode = Some(mode);

            let mut body = BytesMut::new();
            body.put_u8(if mtype == TLOPEN { RLOPEN } else { ROPEN });
            body.put_u16_le(tag);
            qid.encode(&mut body);
            body.put_u32_le(IOUNIT);
            Ok(frame(body))
        }
        TREAD | TREADDIR => {
            let fid = cur.get_u32_le();
            let offset = cur.get_u64_le();
            let count = cur.get_u32_le() as usize;
            let (logical, real, is_dir) = {
                let fid_ent = session
                    .fids
                    .get(&fid)
                    .ok_or_else(|| anyhow!("unknown fid"))?;
                (
                    fid_ent.logical.clone(),
                    fid_ent.real.clone(),
                    fid_ent.logical.is_empty() || fid_ent.real.is_dir(),
                )
            };

            let data = if is_dir {
                encode_dir_entries(session, &logical, &real, offset, count)?
            } else if let Some(ref mut f) = session
                .fids
                .get_mut(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?
                .file
            {
                f.seek(SeekFrom::Start(offset))?;
                let mut buf = vec![0u8; count.min(IOUNIT as usize)];
                let n = f.read(&mut buf)?;
                buf.truncate(n);
                buf
            } else if real.is_file() {
                let mut f = fs::File::open(&real)?;
                f.seek(SeekFrom::Start(offset))?;
                let mut buf = vec![0u8; count.min(IOUNIT as usize)];
                let n = f.read(&mut buf)?;
                buf.truncate(n);
                buf
            } else {
                encode_dir_entries(session, &logical, &real, offset, count)?
            };

            let mut body = BytesMut::new();
            body.put_u8(if mtype == TREADDIR { RREADDIR } else { RREAD });
            body.put_u16_le(tag);
            body.put_u32_le(data.len() as u32);
            body.extend_from_slice(&data);
            Ok(frame(body))
        }
        TWRITE => {
            let fid = cur.get_u32_le();
            let offset = cur.get_u64_le();
            let count = cur.get_u32_le() as usize;
            if cur.remaining() < count {
                bail!("short write data");
            }
            let mut data = vec![0u8; count];
            cur.copy_to_slice(&mut data);

            let repo = session.repo.clone();
            let fid_ent = session
                .fids
                .get_mut(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?;

            if !is_write_allowed(&repo, &fid_ent.real) {
                let path = fid_ent.logical.clone();
                let _ = emit_void_event(&repo, &path, "Twrite outside allowed write root");
                return Ok(rerror(
                    tag,
                    "permission denied: write root is .atom-trail/decisions only",
                ));
            }

            if let Some(parent) = fid_ent.real.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut f = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&fid_ent.real)?;
            f.seek(SeekFrom::Start(offset))?;
            f.write_all(&data)?;
            log_write(&fid_ent.logical, data.len(), 1.0);

            let mut body = BytesMut::new();
            body.put_u8(RWRITE);
            body.put_u16_le(tag);
            body.put_u32_le(data.len() as u32);
            Ok(frame(body))
        }
        TCLUNK => {
            let fid = cur.get_u32_le();
            session.fids.remove(&fid);
            let mut body = BytesMut::new();
            body.put_u8(RCLUNK);
            body.put_u16_le(tag);
            Ok(frame(body))
        }
        TREMOVE => {
            let fid = cur.get_u32_le();
            let fid_ent = session
                .fids
                .get(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?;
            if !is_write_allowed(&session.repo, &fid_ent.real) {
                let path = fid_ent.logical.clone();
                let _ = emit_void_event(&session.repo, &path, "Tremove outside write root");
                return Ok(rerror(tag, "permission denied"));
            }
            if fid_ent.real.exists() {
                if fid_ent.real.is_dir() {
                    fs::remove_dir(&fid_ent.real)?;
                } else {
                    fs::remove_file(&fid_ent.real)?;
                }
            }
            session.fids.remove(&fid);
            let mut body = BytesMut::new();
            body.put_u8(121); // Rremove = 123? Tremove=122 Rremove=123
            // fix: Rremove = 123
            body.clear();
            body.put_u8(123);
            body.put_u16_le(tag);
            Ok(frame(body))
        }
        TSTAT => {
            let fid = cur.get_u32_le();
            let fid_ent = session
                .fids
                .get(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?
                .clone();
            let stat = encode_stat(session, &fid_ent)?;
            let mut body = BytesMut::new();
            body.put_u8(RSTAT);
            body.put_u16_le(tag);
            body.put_u16_le(stat.len() as u16);
            body.extend_from_slice(&stat);
            Ok(frame(body))
        }
        TGETATTR => {
            let fid = cur.get_u32_le();
            let _request_mask = cur.get_u64_le();
            let fid_ent = session
                .fids
                .get(&fid)
                .ok_or_else(|| anyhow!("unknown fid"))?
                .clone();
            let (qid, meta) = getattr_data(session, &fid_ent)?;
            let mut body = BytesMut::new();
            body.put_u8(RGETATTR);
            body.put_u16_le(tag);
            // valid mask
            body.put_u64_le(0x0000_3fff); // basic attrs
            qid.encode(&mut body);
            body.put_u32_le(meta.mode);
            body.put_u32_le(meta.uid);
            body.put_u32_le(meta.gid);
            body.put_u64_le(meta.nlink);
            body.put_u64_le(meta.rdev);
            body.put_u64_le(meta.size);
            body.put_u64_le(meta.blksize);
            body.put_u64_le(meta.blocks);
            // atime sec/nsec, mtime, ctime, btime, gen, data_version
            for _ in 0..4 {
                body.put_u64_le(meta.mtime_sec);
                body.put_u64_le(0);
            }
            body.put_u64_le(0);
            body.put_u64_le(0);
            Ok(frame(body))
        }
        TSTATFS => {
            let _fid = cur.get_u32_le();
            let mut body = BytesMut::new();
            body.put_u8(RSTATFS);
            body.put_u16_le(tag);
            body.put_u32_le(0); // type
            body.put_u32_le(4096); // bsize
            body.put_u64_le(1_000_000); // blocks
            body.put_u64_le(500_000); // bfree
            body.put_u64_le(500_000); // bavail
            body.put_u64_le(100_000); // files
            body.put_u64_le(50_000); // ffree
            body.put_u64_le(0); // fsid
            body.put_u32_le(255); // namelen
            Ok(frame(body))
        }
        TCREATE | TLCREATE | TSETATTR | TWSTAT => {
            // Only allow create under decisions
            Ok(rerror(
                tag,
                "create/setattr restricted — write via Twrite under atom-trail/decisions",
            ))
        }
        _ => Ok(rerror(tag, &format!("unsupported message type {mtype}"))),
    }
}

struct AttrMeta {
    mode: u32,
    uid: u32,
    gid: u32,
    nlink: u64,
    rdev: u64,
    size: u64,
    blksize: u64,
    blocks: u64,
    mtime_sec: u64,
}

fn getattr_data(session: &mut Session, fid: &Fid) -> Result<(Qid, AttrMeta)> {
    if fid.logical.is_empty() {
        let qid = Qid {
            qtype: QTDIR,
            version: 0,
            path: 0,
        };
        return Ok((
            qid,
            AttrMeta {
                mode: 0o040755,
                uid: 1000,
                gid: 1000,
                nlink: 2,
                rdev: 0,
                size: 0,
                blksize: 4096,
                blocks: 0,
                mtime_sec: 0,
            },
        ));
    }
    let meta = fs::metadata(&fid.real)?;
    let qid = session.qid_of(&fid.real);
    let mode = if meta.is_dir() {
        0o040755
    } else {
        0o100644
    };
    let size = meta.len();
    Ok((
        qid,
        AttrMeta {
            mode,
            uid: 1000,
            gid: 1000,
            nlink: 1,
            rdev: 0,
            size,
            blksize: 4096,
            blocks: size.div_ceil(512),
            mtime_sec: meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0),
        },
    ))
}

fn resolve_logical(repo: &Path, logical: &str) -> Result<PathBuf> {
    if logical.is_empty() {
        return Ok(repo.to_path_buf());
    }
    let parts: Vec<&str> = logical.split('/').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() {
        return Ok(repo.to_path_buf());
    }
    let top = parts[0];
    let rest: PathBuf = parts[1..].iter().collect();
    for (name, base, _) in root_entries(repo) {
        if name == top {
            let p = if rest.as_os_str().is_empty() {
                base.clone()
            } else {
                base.join(&rest)
            };
            // prevent escape
            let base_c = base.canonicalize().unwrap_or_else(|_| base.clone());
            if let Ok(pc) = p.canonicalize() {
                if !pc.starts_with(&base_c) {
                    bail!("path escape");
                }
                return Ok(pc);
            }
            return Ok(p);
        }
    }
    bail!("unknown root component {top}");
}

fn encode_dir_entries(
    session: &mut Session,
    logical: &str,
    real: &Path,
    offset: u64,
    count: usize,
) -> Result<Vec<u8>> {
    // Build full listing then slice by offset (byte offset into stream)
    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    if logical.is_empty() {
        for (name, path, _) in root_entries(&session.repo) {
            if path.is_dir()
                || name == "schemas"
                || name == "atom-trail"
                || name == "notebooks"
                || path.exists()
            {
                entries.push((name, path));
            }
        }
    } else if real.is_dir() {
        for ent in fs::read_dir(real)? {
            let ent = ent?;
            let name = ent.file_name().to_string_lossy().into_owned();
            entries.push((name, ent.path()));
        }
    }

    // 9P2000.L readdir entry: qid[13] + offset[8] + type[1] + name[s]
    let mut stream = BytesMut::new();
    let mut off = 1u64;
    for (name, path) in entries {
        let qid = if path.exists() {
            session.qid_of(&path)
        } else {
            Qid {
                qtype: QTDIR,
                version: 0,
                path: session.path_id(&path),
            }
        };
        let mut ent = BytesMut::new();
        qid.encode(&mut ent);
        ent.put_u64_le(off);
        ent.put_u8(if path.is_dir() || !path.exists() {
            4
        } else {
            8
        }); // DT_DIR=4 DT_REG=8
        put_string(&mut ent, &name);
        stream.extend_from_slice(&ent);
        off += 1;
    }

    let bytes = stream.freeze();
    if offset as usize >= bytes.len() {
        return Ok(vec![]);
    }
    let end = (offset as usize + count).min(bytes.len());
    Ok(bytes[offset as usize..end].to_vec())
}

fn encode_stat(session: &mut Session, fid: &Fid) -> Result<Vec<u8>> {
    // classic 9P stat
    let name = if fid.logical.is_empty() {
        "/".into()
    } else {
        fid.logical
            .rsplit('/')
            .next()
            .unwrap_or(&fid.logical)
            .to_string()
    };
    let (qid, meta) = getattr_data(session, fid)?;
    let mut st = BytesMut::new();
    // size placeholder
    let size_pos = st.len();
    st.put_u16_le(0);
    st.put_u16_le(0); // type
    st.put_u32_le(0); // dev
    qid.encode(&mut st);
    st.put_u32_le(meta.mode);
    st.put_u32_le(meta.mtime_sec as u32); // atime
    st.put_u32_le(meta.mtime_sec as u32); // mtime
    st.put_u64_le(meta.size);
    put_string(&mut st, &name);
    put_string(&mut st, "styx");
    put_string(&mut st, "styx");
    put_string(&mut st, "styx");
    let size = (st.len() - 2) as u16;
    st[size_pos..size_pos + 2].copy_from_slice(&size.to_le_bytes());
    Ok(st.to_vec())
}

// Need Clone for Fid - file handle not cloneable; re-open on demand
impl Clone for Fid {
    fn clone(&self) -> Self {
        Self {
            real: self.real.clone(),
            logical: self.logical.clone(),
            omode: self.omode,
            file: None,
        }
    }
}

async fn read_message(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut sz = [0u8; 4];
    stream.read_exact(&mut sz).await?;
    let size = u32::from_le_bytes(sz) as usize;
    if size < 7 || size > 1024 * 1024 {
        bail!("invalid message size {size}");
    }
    let mut buf = vec![0u8; size];
    buf[0..4].copy_from_slice(&sz);
    stream.read_exact(&mut buf[4..]).await?;
    Ok(buf)
}

async fn handle_client(mut stream: TcpStream, repo: PathBuf) -> Result<()> {
    let peer = stream.peer_addr().ok();
    info!("9P session from {:?}", peer);
    let session = Arc::new(Mutex::new(Session::new(repo)));
    loop {
        let msg = match read_message(&mut stream).await {
            Ok(m) => m,
            Err(e) => {
                warn!("session end: {e}");
                break;
            }
        };
        let resp = {
            let mut s = session.lock().await;
            match handle(&mut s, &msg) {
                Ok(r) => r,
                Err(e) => {
                    let tag = if msg.len() >= 7 {
                        u16::from_le_bytes([msg[5], msg[6]])
                    } else {
                        NOTAG
                    };
                    rerror(tag, &format!("{e}"))
                }
            }
        };
        stream.write_all(&resp).await?;
    }
    Ok(())
}

pub async fn serve(cfg: BookshelfConfig) -> Result<()> {
    // ensure write root exists
    fs::create_dir_all(cfg.repo_root.join(".atom-trail/decisions"))?;
    for (_, path, _) in root_entries(&cfg.repo_root) {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
    }

    let listener = TcpListener::bind(&cfg.listen)
        .await
        .with_context(|| format!("bind {}", cfg.listen))?;
    info!(
        "styx-bookshelf 9P2000.L listening on {} root={}",
        cfg.listen,
        cfg.repo_root.display()
    );
    info!(
        "read roots: atom-trail, schemas, notebooks (+ verification_certificates if present)"
    );
    info!("write root: .atom-trail/decisions/ ONLY");

    loop {
        let (sock, _) = listener.accept().await?;
        let repo = cfg.repo_root.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(sock, repo).await {
                warn!("client error: {e:#}");
            }
        });
    }
}
