use anyhow::Result;
use std::time::{Duration, Instant};
use url::Url;
use reqwest::blocking::Client;
use std::collections::HashMap;

pub struct MirrorSelector {
    client: Client,
    timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct Mirror {
    pub url: Url,
    pub name: String,
    pub country: String,
    pub speed: Option<Duration>,
    pub last_check: Option<Instant>,
    pub is_available: bool,
}

impl MirrorSelector {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
            timeout: Duration::from_secs(5),
        }
    }

    pub fn select_fastest_mirror(&self, mirrors: &[Mirror]) -> Option<Mirror> {
        let mut fastest = None;
        let mut best_speed = Duration::from_secs(60);

        for mirror in mirrors {
            if mirror.is_available {
                if let Some(speed) = mirror.speed {
                    if speed < best_speed {
                        best_speed = speed;
                        fastest = Some(mirror.clone());
                    }
                }
            }
        }

        fastest
    }

    pub fn check_mirrors(&self, mirrors: &mut [Mirror]) -> Result<()> {
        for mirror in mirrors.iter_mut() {
            let start = Instant::now();
            let url = mirror.url.join("status")?;
            
            match self.client.head(url.as_str()).send() {
                Ok(response) => {
                    let elapsed = start.elapsed();
                    mirror.speed = Some(elapsed);
                    mirror.is_available = response.status().is_success();
                    mirror.last_check = Some(Instant::now());
                }
                Err(_) => {
                    mirror.is_available = false;
                    mirror.last_check = Some(Instant::now());
                }
            }
        }

        Ok(())
    }

    pub fn get_mirrors_by_country(&self, mirrors: &[Mirror], country: &str) -> Vec<Mirror> {
        mirrors
            .iter()
            .filter(|m| m.country.eq_ignore_ascii_case(country) && m.is_available)
            .cloned()
            .collect()
    }

    pub fn get_mirror_status(&self, mirrors: &[Mirror]) -> HashMap<String, bool> {
        let mut status = HashMap::new();
        
        for mirror in mirrors {
            status.insert(mirror.name.clone(), mirror.is_available);
        }
        
        status
    }
} 