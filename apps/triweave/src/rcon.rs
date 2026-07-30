//! Minecraft RCON client for triweave.
//!
//! Source RCON protocol over TCP. Used by:
//! - `triweave deploy amazon-room` (build structure)
//! - `triweave search <query>` (render holograms)
//! - `triweave doctor` (ping RCON health check)
//!
//! Reference: https://wiki.vg/RCON
//! Conservation: alpha + omega = 15

use anyhow::{bail, Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const PACKET_TYPE_LOGIN: i32 = 3;
const PACKET_TYPE_COMMAND: i32 = 2;

/// Connected RCON client.
pub struct RconClient {
    stream: TcpStream,
    next_id: i32,
}

impl RconClient {
    /// Connect and authenticate.
    pub async fn connect(host: &str, port: u16, password: &str) -> Result<Self> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr)
            .await
            .context(format!("RCON: cannot connect to {}", addr))?;
        let mut client = Self { stream, next_id: 1 };

        let resp = client.send_packet(PACKET_TYPE_LOGIN, password).await?;
        if resp.packet_type == -1 {
            bail!("RCON authentication failed at {}", addr);
        }

        tracing::info!("RCON connected to {}", addr);
        Ok(client)
    }

    /// Send a command and return the response text.
    pub async fn command(&mut self, cmd: &str) -> Result<String> {
        let resp = self.send_packet(PACKET_TYPE_COMMAND, cmd).await?;
        Ok(resp.body)
    }

    /// Execute a batch of commands with optional delay between them.
    /// Returns (successes, failures).
    pub async fn execute_batch(
        &mut self,
        commands: &[String],
        delay_ms: u64,
    ) -> (usize, usize) {
        let mut ok = 0usize;
        let mut fail = 0usize;

        for cmd in commands {
            match self.command(cmd).await {
                Ok(_) => ok += 1,
                Err(e) => {
                    tracing::warn!("RCON command failed: {} — {}", cmd, e);
                    fail += 1;
                }
            }
            if delay_ms > 0 {
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
            }
        }

        (ok, fail)
    }

    /// Ping — send a harmless command to check connectivity.
    pub async fn ping(&mut self) -> bool {
        self.command("/list").await.is_ok()
    }

    async fn send_packet(&mut self, packet_type: i32, body: &str) -> Result<RconPacket> {
        let id = self.next_id;
        self.next_id += 1;

        let body_bytes = body.as_bytes();
        let length = (4 + 4 + body_bytes.len() + 2) as i32;

        // Write
        self.stream.write_i32_le(length).await?;
        self.stream.write_i32_le(id).await?;
        self.stream.write_i32_le(packet_type).await?;
        self.stream.write_all(body_bytes).await?;
        self.stream.write_all(&[0, 0]).await?;
        self.stream.flush().await?;

        // Read
        let resp_len = self.stream.read_i32_le().await? as usize;
        let mut buf = vec![0u8; resp_len];
        self.stream.read_exact(&mut buf).await?;

        let resp_id = i32::from_le_bytes(buf[0..4].try_into()?);
        let resp_type = i32::from_le_bytes(buf[4..8].try_into()?);
        let resp_body =
            String::from_utf8_lossy(&buf[8..resp_len.saturating_sub(2)]).to_string();

        Ok(RconPacket {
            _id: resp_id,
            packet_type: resp_type,
            body: resp_body,
        })
    }
}

struct RconPacket {
    _id: i32,
    packet_type: i32,
    body: String,
}
