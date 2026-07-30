//! Minimal Minecraft RCON client (Source RCON protocol).
//!
//! Reference: https://wiki.vg/RCON

use anyhow::{bail, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const PACKET_TYPE_LOGIN: i32 = 3;
const PACKET_TYPE_COMMAND: i32 = 2;
const PACKET_TYPE_RESPONSE: i32 = 0;

/// A connected RCON client.
pub struct RconClient {
    stream: TcpStream,
    next_id: i32,
}

impl RconClient {
    /// Connect and authenticate to an RCON server.
    pub async fn connect(addr: &str, password: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let mut client = Self { stream, next_id: 1 };

        // Authenticate
        let resp = client.send_packet(PACKET_TYPE_LOGIN, password).await?;
        if resp.packet_type == -1 {
            bail!("RCON authentication failed");
        }

        Ok(client)
    }

    /// Send a command and return the response body.
    pub async fn command(&self, cmd: &str) -> Result<String> {
        // RCON is not concurrent-safe on one connection, but for our
        // single-threaded use this is fine. We use an unsafe cast to
        // allow &self since the caller serialises access.
        let this = unsafe { &mut *(self as *const Self as *mut Self) };
        let resp = this.send_packet(PACKET_TYPE_COMMAND, cmd).await?;
        Ok(resp.body)
    }

    async fn send_packet(&mut self, packet_type: i32, body: &str) -> Result<RconPacket> {
        let id = self.next_id;
        self.next_id += 1;

        let body_bytes = body.as_bytes();
        let length = 4 + 4 + body_bytes.len() + 2; // id + type + body + 2 null terminators

        // Write packet
        self.stream.write_i32_le(length as i32).await?;
        self.stream.write_i32_le(id).await?;
        self.stream.write_i32_le(packet_type).await?;
        self.stream.write_all(body_bytes).await?;
        self.stream.write_all(&[0, 0]).await?;
        self.stream.flush().await?;

        // Read response
        let resp_len = self.stream.read_i32_le().await? as usize;
        let mut buf = vec![0u8; resp_len];
        self.stream.read_exact(&mut buf).await?;

        let resp_id = i32::from_le_bytes(buf[0..4].try_into()?);
        let resp_type = i32::from_le_bytes(buf[4..8].try_into()?);
        let resp_body = String::from_utf8_lossy(&buf[8..resp_len.saturating_sub(2)]).to_string();

        Ok(RconPacket {
            id: resp_id,
            packet_type: resp_type,
            body: resp_body,
        })
    }
}

#[derive(Debug)]
struct RconPacket {
    id: i32,
    packet_type: i32,
    body: String,
}
