#!/usr/bin/env python3
"""Minimal 9P2000.L client smoke for GB-03 (no kernel mount required)."""
import hashlib
import os
import socket
import struct
import sys
from pathlib import Path

HOST, PORT = "127.0.0.1", 5640
TVERSION, RVERSION = 100, 101
TATTACH, RATTACH = 104, 105
RERROR = 107
TWALK, RWALK = 110, 111
TOPEN, ROPEN = 112, 113
TLOPEN, RLOPEN = 12, 13
TREAD, RREAD = 116, 117
TWRITE, RWRITE = 118, 119
TCLUNK, RCLUNK = 120, 121


def pstring(s: str) -> bytes:
    b = s.encode()
    return struct.pack("<H", len(b)) + b


def read_msg(sock: socket.socket) -> bytes:
    hdr = sock.recv(4)
    if len(hdr) < 4:
        raise RuntimeError("short size")
    (size,) = struct.unpack("<I", hdr)
    body = b""
    while len(body) < size - 4:
        chunk = sock.recv(size - 4 - len(body))
        if not chunk:
            break
        body += chunk
    return hdr + body


def send_msg(sock: socket.socket, mtype: int, tag: int, payload: bytes) -> bytes:
    body = struct.pack("<BH", mtype, tag) + payload
    msg = struct.pack("<I", 4 + len(body)) + body
    sock.sendall(msg)
    return read_msg(sock)


def main() -> int:
    root = Path(os.environ.get("LOGOS_ROOT", Path.home() / "LogOS"))
    schema = root / "docs/schemas/v0.1/certificate.schema.json"
    expect = hashlib.sha256(schema.read_bytes()).hexdigest()

    s = socket.create_connection((HOST, PORT), timeout=5)
    # Tversion
    r = send_msg(s, TVERSION, 0xFFFF, struct.pack("<I", 8192) + pstring("9P2000.L"))
    assert r[4] == RVERSION, f"version failed {r[4]}"
    # Tattach fid=1
    r = send_msg(s, TATTACH, 1, struct.pack("<II", 1, 0xFFFFFFFF) + pstring("toolated") + pstring(""))
    assert r[4] == RATTACH, f"attach failed type={r[4]}"
    # Twalk fid=1 newfid=2 -> schemas
    r = send_msg(s, TWALK, 2, struct.pack("<IIH", 1, 2, 1) + pstring("schemas"))
    assert r[4] == RWALK, f"walk schemas failed {r[4]}"
    # Twalk -> certificate.schema.json
    r = send_msg(
        s,
        TWALK,
        3,
        struct.pack("<IIH", 2, 3, 1) + pstring("certificate.schema.json"),
    )
    assert r[4] == RWALK, f"walk file failed {r[4]}"
    # Topen fid=3 mode=0
    r = send_msg(s, TOPEN, 4, struct.pack("<IB", 3, 0))
    if r[4] not in (ROPEN, RLOPEN):
        # try Tlopen O_RDONLY
        r = send_msg(s, TLOPEN, 4, struct.pack("<II", 3, 0))
    assert r[4] in (ROPEN, RLOPEN), f"open failed type={r[4]} body={r!r}"
    # Tread
    data = b""
    off = 0
    while True:
        r = send_msg(s, TREAD, 5, struct.pack("<IQI", 3, off, 4096))
        assert r[4] == RREAD, f"read failed {r[4]}"
        (n,) = struct.unpack_from("<I", r, 7)
        chunk = r[11 : 11 + n]
        if not chunk:
            break
        data += chunk
        off += len(chunk)
        if n < 4096:
            break
    got = hashlib.sha256(data).hexdigest()
    print(f"SC-read-hash match={got == expect} bytes={len(data)}")
    if got != expect:
        print("expected", expect)
        print("got     ", got)
        return 1

    # Disallowed write: walk schemas, open write
    r = send_msg(s, TWALK, 6, struct.pack("<IIH", 2, 4, 1) + pstring("evil.txt"))
    # may fail walk if not exists — create path via open on new name under schemas
    # Use walk to schemas (fid 2) and Twrite after open with O_WRONLY
    r = send_msg(s, TWALK, 7, struct.pack("<IIH", 2, 5, 0))  # clone schemas fid
    r = send_msg(s, TOPEN, 8, struct.pack("<IB", 5, 1))  # OWRITE on directory should fail or
    # Instead Twrite on file fid after walk to schema file with write mode
    r = send_msg(s, TWALK, 9, struct.pack("<IIH", 2, 6, 1) + pstring("certificate.schema.json"))
    r = send_msg(s, TOPEN, 10, struct.pack("<IB", 6, 1))  # write open
    if r[4] == RERROR:
        print("SC-gate-open: Rerror on write-open (good)")
        err = r[7:]
        print("  rerror payload", err[:80])
    else:
        # try Twrite
        r = send_msg(s, TWRITE, 11, struct.pack("<IQI", 6, 0, 4) + b"nope")
        if r[4] == RERROR:
            print("SC-gate-write: Rerror on Twrite (good)")
        else:
            print("SC-gate FAIL: write succeeded type", r[4])
            return 1

    # VOID event present?
    voids = list((root / ".atom-trail").glob("VOID-9P-*.json"))
    print(f"VOID events: {len(voids)} latest={voids[-1].name if voids else None}")
    if not voids:
        print("SC-gate VOID missing")
        return 1

    send_msg(s, TCLUNK, 12, struct.pack("<I", 3))
    print("SMOKE_OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
