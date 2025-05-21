use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::time::Duration;

pub struct ProgressReporter {
    multi_progress: MultiProgress,
}

impl ProgressReporter {
    pub fn new() -> Self {
        Self {
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn create_progress_bar(&self, total: u64, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(total));
        
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("=> "),
        );
        
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        
        pb
    }

    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        
        pb
    }

    pub fn create_download_progress_bar(&self, total: u64, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(total));
        
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.green/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                .unwrap()
                .progress_chars("=> "),
        );
        
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        
        pb
    }

    pub fn create_indefinite_progress_bar(&self, message: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new_spinner());
        
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        
        pb.set_message(message.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        
        pb
    }
} 