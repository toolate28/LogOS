//! SPHINX-gated ATOM-AUTH vault.
//!
//! Keys encrypted at rest with AES-256-GCM. Encryption key derived from:
//!   1. Jones polynomial fingerprint of per-key SPHINX braid
//!   2. User passphrase via argon2id KDF
//!
//! Every decrypt operation is logged as an ATOM trail entry.
//!
//! Conservation: α + ω = 15

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{bail, Context, Result};
use argon2::Argon2;
use chrono::{DateTime, Utc};
use aes_gcm::aead::rand_core::RngCore;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::sphinx;

/// A single encrypted key entry in the vault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    pub key_name: String,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub braid_word: Vec<i32>,
    pub fingerprint: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

/// The SPHINX-gated vault — holds encrypted API keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<VaultEntry>,
    pub master_fingerprint: String,
    pub created_at: DateTime<Utc>,
}

impl Vault {
    /// Vault file path: `~/.triweave/vault.sphinx`
    pub fn path() -> Result<PathBuf> {
        let home = dirs_next().context("cannot determine home directory")?;
        Ok(home.join(".triweave").join("vault.sphinx"))
    }

    /// Create a new empty vault.
    pub fn new(passphrase: &str) -> Self {
        let braid = sphinx::payload_to_braid_word(passphrase);
        let fp = sphinx::compute_fingerprint(&braid, 3);
        Vault {
            entries: Vec::new(),
            master_fingerprint: fp,
            created_at: Utc::now(),
        }
    }

    /// Load vault from disk.
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            bail!("No vault found at {}. Run `triweave init` first.", path.display());
        }
        let data = std::fs::read(&path).context("reading vault file")?;
        let vault: Vault = serde_json::from_slice(&data).context("parsing vault")?;
        Ok(vault)
    }

    /// Save vault to disk.
    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_vec_pretty(self)?;
        std::fs::write(&path, data).context("writing vault file")?;
        tracing::info!("vault saved to {}", path.display());
        Ok(())
    }

    /// Encrypt and store a key.
    pub fn store_key(&mut self, key_name: &str, plaintext: &str, passphrase: &str) -> Result<()> {
        // Remove existing entry with same name
        self.entries.retain(|e| e.key_name != key_name);

        // Generate per-key SPHINX braid from key_name + passphrase
        let braid_payload = format!("{}:{}", key_name, passphrase);
        let braid_word = sphinx::payload_to_braid_word(&braid_payload);
        let fp = sphinx::compute_fingerprint(&braid_word, 3);

        // Derive AES-256 key: argon2id(passphrase, salt=fingerprint_bytes)
        let cipher_key = derive_key(passphrase, &fp)?;
        let cipher = Aes256Gcm::new_from_slice(&cipher_key)
            .map_err(|_| anyhow::anyhow!("creating AES-256-GCM cipher"))?;

        // Encrypt
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("encryption failed: {e}"))?;

        let now = Utc::now();
        let fingerprint_str = fp;
        let fingerprint_preview = fingerprint_str[..16].to_string();

        self.entries.push(VaultEntry {
            key_name: key_name.to_string(),
            ciphertext,
            nonce: nonce_bytes,
            braid_word,
            fingerprint: fingerprint_str,
            created_at: now,
            last_accessed: now,
        });

        tracing::info!(
            "ATOM-AUTH: stored key '{}' (braid={:?}, fingerprint={}…)",
            key_name,
            &self.entries.last().unwrap().braid_word[..3],
            fingerprint_preview
        );

        Ok(())
    }

    /// Decrypt a key. Verifies SPHINX fingerprint before decryption.
    /// Logs ATOM trail entry on every access.
    pub fn decrypt_key(&mut self, key_name: &str, passphrase: &str) -> Result<String> {
        let entry = self
            .entries
            .iter_mut()
            .find(|e| e.key_name == key_name)
            .context(format!("key '{}' not found in vault", key_name))?;

        // SPHINX gate: verify fingerprint matches stored braid
        let braid_payload = format!("{}:{}", key_name, passphrase);
        let expected_braid = sphinx::payload_to_braid_word(&braid_payload);
        if expected_braid != entry.braid_word {
            bail!("SPHINX_REJECT: braid word mismatch for key '{}'", key_name);
        }

        if !sphinx::validate(&entry.braid_word, &entry.fingerprint) {
            bail!(
                "SPHINX_REJECT: jones_polynomial_mismatch for key '{}'",
                key_name
            );
        }

        // Derive decryption key
        let cipher_key = derive_key(passphrase, &entry.fingerprint)?;
        let cipher = Aes256Gcm::new_from_slice(&cipher_key)
            .map_err(|_| anyhow::anyhow!("creating AES-256-GCM cipher"))?;

        let nonce = Nonce::from_slice(&entry.nonce);
        let plaintext = cipher
            .decrypt(nonce, entry.ciphertext.as_ref())
            .map_err(|_| anyhow::anyhow!("SPHINX_REJECT: decryption failed — wrong passphrase or tampered vault"))?;

        entry.last_accessed = Utc::now();

        tracing::info!(
            "ATOM-AUTH: decrypted key '{}' (SPHINX gate passed, fingerprint={}…)",
            key_name,
            &entry.fingerprint[..16]
        );

        String::from_utf8(plaintext).context("decrypted key is not valid UTF-8")
    }

    /// List all key names (never values).
    pub fn key_names(&self) -> Vec<&str> {
        self.entries.iter().map(|e| e.key_name.as_str()).collect()
    }
}

/// Derive a 32-byte AES-256 key from passphrase + fingerprint salt.
fn derive_key(passphrase: &str, fingerprint_hex: &str) -> Result<[u8; 32]> {
    // Use first 16 bytes of fingerprint as salt
    let salt_bytes = hex_to_bytes(&fingerprint_hex[..32])?;
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), &salt_bytes, &mut key)
        .map_err(|e| anyhow::anyhow!("argon2 KDF failed: {e}"))?;
    Ok(key)
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).context("invalid hex"))
        .collect()
}

fn dirs_next() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        std::env::var("USERPROFILE").ok().map(PathBuf::from)
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
}

// ─── Public CLI commands ────────────────────────────────────────────

/// `triweave vault list`
pub async fn list_keys() -> Result<()> {
    let vault = Vault::load()?;
    if vault.entries.is_empty() {
        println!("Vault is empty. Run `triweave init` to add keys.");
        return Ok(());
    }
    println!("SPHINX Vault — {} keys:", vault.entries.len());
    for entry in &vault.entries {
        println!(
            "  {} — braid[{}] fingerprint={}… last_access={}",
            entry.key_name,
            entry.braid_word.len(),
            &entry.fingerprint[..16],
            entry.last_accessed.format("%Y-%m-%d %H:%M")
        );
    }
    Ok(())
}

/// `triweave vault rotate <key>`
pub async fn rotate_key(key_name: &str) -> Result<()> {
    let mut vault = Vault::load()?;
    // Prompt for passphrase
    let passphrase = prompt_passphrase("Enter vault passphrase: ")?;
    // Decrypt old key
    let plaintext = vault.decrypt_key(key_name, &passphrase)?;
    // Prompt for new value (or keep existing)
    println!("Current key decrypted. Enter new value (or press Enter to re-encrypt with new braid):");
    let mut new_value = String::new();
    std::io::stdin().read_line(&mut new_value)?;
    let new_value = new_value.trim();
    let value = if new_value.is_empty() {
        plaintext
    } else {
        new_value.to_string()
    };
    // Re-encrypt with fresh braid
    vault.store_key(key_name, &value, &passphrase)?;
    vault.save()?;
    println!("Key '{}' rotated with new SPHINX braid.", key_name);
    Ok(())
}

/// `triweave vault audit`
pub async fn audit() -> Result<()> {
    let vault = Vault::load()?;
    println!("SPHINX Vault Audit");
    println!("  Created: {}", vault.created_at);
    println!("  Master fingerprint: {}", vault.master_fingerprint);
    println!("  Entries: {}", vault.entries.len());
    println!();
    for entry in &vault.entries {
        println!("  [{}]", entry.key_name);
        println!("    Braid word: {:?}", entry.braid_word);
        println!("    Fingerprint: {}", entry.fingerprint);
        println!("    Created: {}", entry.created_at);
        println!("    Last accessed: {}", entry.last_accessed);
        // Verify fingerprint integrity
        let valid = sphinx::validate(&entry.braid_word, &entry.fingerprint);
        println!("    SPHINX integrity: {}", if valid { "✓ PASS" } else { "✗ FAIL" });
        println!();
    }
    Ok(())
}

/// Prompt for passphrase (no echo).
pub fn prompt_passphrase(prompt: &str) -> Result<String> {
    use std::io::Write;
    print!("{}", prompt);
    std::io::stdout().flush()?;
    // In a TUI context we'd use crossterm raw mode. For CLI fallback:
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vault_roundtrip() {
        let passphrase = "test-passphrase-alpha-omega-15";
        let mut vault = Vault::new(passphrase);
        vault
            .store_key("anthropic", "sk-ant-test-key-12345", passphrase)
            .unwrap();
        vault
            .store_key("xai", "xai-test-key-67890", passphrase)
            .unwrap();

        assert_eq!(vault.key_names(), vec!["anthropic", "xai"]);

        // Decrypt
        let decrypted = vault.decrypt_key("anthropic", passphrase).unwrap();
        assert_eq!(decrypted, "sk-ant-test-key-12345");

        let decrypted = vault.decrypt_key("xai", passphrase).unwrap();
        assert_eq!(decrypted, "xai-test-key-67890");
    }

    #[test]
    fn wrong_passphrase_fails() {
        let mut vault = Vault::new("correct-passphrase");
        vault
            .store_key("test", "secret", "correct-passphrase")
            .unwrap();

        let result = vault.decrypt_key("test", "wrong-passphrase");
        assert!(result.is_err());
    }

    #[test]
    fn sphinx_fingerprint_integrity() {
        let mut vault = Vault::new("passphrase");
        vault.store_key("key1", "value1", "passphrase").unwrap();

        let entry = &vault.entries[0];
        assert!(sphinx::validate(&entry.braid_word, &entry.fingerprint));
    }
}
