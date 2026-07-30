//! RCON Client — Source RCON Protocol implementation for Minecraft
//!
//! Packet: [size:i32][id:i32][type:i32][body:string][pad:0x00][pad:0x00]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

const SERVERDATA_AUTH: i32 = 3;
const SERVERDATA_EXECCOMMAND: i32 = 2;

pub struct RconClient {
    stream: Arc<Mutex<TcpStream>>,
    request_id: AtomicI32,
}

impl RconClient {
    pub async fn connect(host: &str, port: u16, password: &str) -> Result<Self, String> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| format!("TCP connect to {} failed: {}", addr, e))?;

        let client = Self {
            stream: Arc::new(Mutex::new(stream)),
            request_id: AtomicI32::new(1),
        };

        // Authenticate
        let auth_resp = client.send_raw(0, SERVERDATA_AUTH, password).await?;
        if auth_resp.0 == -1 {
            return Err("RCON authentication failed — check password".into());
        }

        Ok(client)
    }

    pub async fn send(&self, command: &str) -> Result<String, String> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let (resp_id, body) = self.send_raw(id, SERVERDATA_EXECCOMMAND, command).await?;

        if resp_id == id {
            Ok(body)
        } else {
            Err(format!("Response ID mismatch: expected {}, got {}", id, resp_id))
        }
    }

    async fn send_raw(&self, id: i32, ptype: i32, body: &str) -> Result<(i32, String), String> {
        let mut stream = self.stream.lock().await;

        // Encode packet
        let body_bytes = body.as_bytes();
        let size = 4 + 4 + body_bytes.len() as i32 + 2; // id + type + body + 2 null terminators

        let mut packet = Vec::with_capacity((4 + size) as usize);
        packet.extend_from_slice(&size.to_le_bytes());
        packet.extend_from_slice(&id.to_le_bytes());
        packet.extend_from_slice(&ptype.to_le_bytes());
        packet.extend_from_slice(body_bytes);
        packet.push(0);
        packet.push(0);

        stream.write_all(&packet).await
            .map_err(|e| format!("RCON write failed: {}", e))?;

        // Read response
        let mut size_buf = [0u8; 4];
        stream.read_exact(&mut size_buf).await
            .map_err(|e| format!("RCON read size failed: {}", e))?;
        let resp_size = i32::from_le_bytes(size_buf) as usize;

        let mut resp_buf = vec![0u8; resp_size];
        stream.read_exact(&mut resp_buf).await
            .map_err(|e| format!("RCON read body failed: {}", e))?;

        let resp_id = i32::from_le_bytes([resp_buf[0], resp_buf[1], resp_buf[2], resp_buf[3]]);
        // Skip type (4 bytes), read body until first null
        let body_start = 8;
        let body_end = resp_buf[body_start..].iter()
            .position(|&b| b == 0)
            .map(|p| body_start + p)
            .unwrap_or(resp_buf.len());
        let resp_body = String::from_utf8_lossy(&resp_buf[body_start..body_end]).to_string();

        Ok((resp_id, resp_body))
    }
}
