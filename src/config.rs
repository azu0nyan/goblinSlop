use std::env;
use std::path::Path;

use anyhow::{Result, anyhow};

/// Application configuration loaded from environment variables.
///
/// | Variable | Default | Description |
/// |----------|---------|-------------|
/// | `GOBLIN_HOST` | `0.0.0.0` | Server bind address |
/// | `GOBLIN_PORT` | `3000` | Server port |
/// | `GOBLIN_CONTENT_DIR` | `data/content` | Directory containing content JSON files |
/// | `GOBLIN_STATIC_DIR` | `static` | Directory containing static assets |
/// | `GOBLIN_BASE_URL` | `https://goblin.geno.su` | Base URL for canonical links & sitemap |
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub content_dir: String,
    pub static_dir: String,
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            host: env::var("GOBLIN_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("GOBLIN_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000),
            content_dir: env::var("GOBLIN_CONTENT_DIR")
                .unwrap_or_else(|_| "data/content".to_string()),
            static_dir: env::var("GOBLIN_STATIC_DIR").unwrap_or_else(|_| "static".to_string()),
            base_url: env::var("GOBLIN_BASE_URL")
                .unwrap_or_else(|_| "https://goblin.geno.su".to_string()),
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Fail fast at startup if config values are unusable. The guide's
    /// configuration chapter calls this out as a "must" — surface broken
    /// settings before the server begins binding sockets and loading data.
    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(anyhow!("GOBLIN_HOST must not be empty"));
        }
        if self.base_url.is_empty() {
            return Err(anyhow!("GOBLIN_BASE_URL must not be empty"));
        }
        if !(self.base_url.starts_with("http://") || self.base_url.starts_with("https://")) {
            return Err(anyhow!(
                "GOBLIN_BASE_URL must start with http:// or https:// (got `{}`)",
                self.base_url
            ));
        }
        if !Path::new(&self.content_dir).is_dir() {
            return Err(anyhow!(
                "GOBLIN_CONTENT_DIR `{}` does not exist or is not a directory",
                self.content_dir
            ));
        }
        if !Path::new(&self.static_dir).is_dir() {
            return Err(anyhow!(
                "GOBLIN_STATIC_DIR `{}` does not exist or is not a directory",
                self.static_dir
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(overrides: impl FnOnce(&mut Config)) -> Config {
        let mut c = Config {
            host: "0.0.0.0".into(),
            port: 3000,
            content_dir: "data/content".into(),
            static_dir: "static".into(),
            base_url: "https://example.test".into(),
        };
        overrides(&mut c);
        c
    }

    #[test]
    fn validate_accepts_real_dirs() {
        assert!(cfg(|_| {}).validate().is_ok());
    }

    #[test]
    fn validate_rejects_missing_content_dir() {
        let c = cfg(|c| c.content_dir = "/does/not/exist".into());
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_bad_base_url() {
        let c = cfg(|c| c.base_url = "goblin.example".into());
        assert!(c.validate().is_err());
    }

    #[test]
    fn validate_rejects_empty_host() {
        let c = cfg(|c| c.host = String::new());
        assert!(c.validate().is_err());
    }
}
