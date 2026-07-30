//! Voice Engine — TTS synthesis via Lyria 3 / local engine
//!
//! Synthesizes text to audio for Discord voice channel playback.
//! Pipeline: Text → Lyria 3 (local) → PCM → Songbird → Voice Channel

use std::env;

pub enum VoiceEngine {
    Lyria3,
    Local,
    Disabled,
}

impl VoiceEngine {
    pub fn from_env() -> Self {
        match env::var("VOICE_ENGINE").as_deref() {
            Ok("lyria") | Ok("lyria-3") => Self::Lyria3,
            Ok("local") => Self::Local,
            _ => Self::Disabled,
        }
    }
}

/// Synthesize text to a temporary WAV file path.
/// Returns the path to the generated audio file.
pub async fn synthesize(text: &str, engine: &VoiceEngine) -> Result<String, String> {
    match engine {
        VoiceEngine::Lyria3 => {
            // TODO: Call local Lyria 3 endpoint
            // POST http://localhost:8090/synthesize { "text": "...", "voice": "nexus" }
            Err("Lyria 3 endpoint not yet connected".into())
        }
        VoiceEngine::Local => {
            // Fallback: use system TTS (espeak/piper)
            let output_path = format!(
                "{}/nexus_voice_{}.wav",
                env::temp_dir().display(),
                chrono::Utc::now().timestamp()
            );

            let status = tokio::process::Command::new("piper")
                .args(["--model", "en_US-lessac-medium", "--output_file", &output_path])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    use tokio::io::AsyncWriteExt;
                    let stdin = child.stdin.as_mut().unwrap();
                    let text = text.to_string();
                    tokio::spawn(async move {
                        let _ = stdin.write_all(text.as_bytes()).await;
                    });
                    Ok(child)
                });

            match status {
                Ok(mut child) => {
                    let _ = child.wait().await;
                    Ok(output_path)
                }
                Err(e) => Err(format!("Local TTS failed: {}", e)),
            }
        }
        VoiceEngine::Disabled => {
            Err("Voice engine disabled — set VOICE_ENGINE=local or lyria-3".into())
        }
    }
}
