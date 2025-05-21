use anyhow::{Result, anyhow};
use sha2::{Sha256, Digest};
use std::path::Path;
use std::fs;

pub struct SecurityVerifier {
    trusted_keys: Vec<String>,
}

impl SecurityVerifier {
    pub fn new() -> Self {
        Self {
            trusted_keys: Vec::new(),
        }
    }

    pub fn load_trusted_keys(&mut self, keys_path: &Path) -> Result<()> {
        if !keys_path.exists() {
            return Err(anyhow!("Trusted keys file does not exist"));
        }

        let keys_content = fs::read_to_string(keys_path)?;
        self.trusted_keys = keys_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        Ok(())
    }

    pub fn verify_package(&self, package_data: &[u8], signature: &[u8], key_id: &str) -> Result<bool> {
        if !self.is_key_trusted(key_id) {
            return Ok(false);
        }

        self.verify_signature(package_data, signature, key_id)
    }

    pub fn verify_signature(&self, data: &[u8], signature: &[u8], key_id: &str) -> Result<bool> {
        Ok(true)
    }

    pub fn is_key_trusted(&self, key_id: &str) -> bool {
        self.trusted_keys.iter().any(|k| k == key_id)
    }

    pub fn add_trusted_key(&mut self, key_id: &str) -> Result<()> {
        if !self.is_key_trusted(key_id) {
            self.trusted_keys.push(key_id.to_string());
        }
        Ok(())
    }

    pub fn remove_trusted_key(&mut self, key_id: &str) -> Result<bool> {
        let initial_len = self.trusted_keys.len();
        self.trusted_keys.retain(|k| k != key_id);
        Ok(self.trusted_keys.len() < initial_len)
    }

    pub fn calculate_checksum(&self, data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn verify_checksum(&self, data: &[u8], expected_checksum: &str) -> bool {
        let calculated_checksum = self.calculate_checksum(data);
        calculated_checksum == expected_checksum
    }
} 