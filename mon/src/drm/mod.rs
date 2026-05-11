use std::fs::OpenOptions;
use std::os::fd::{AsFd, BorrowedFd};
use drm::control::Device;

pub struct Card(std::fs::File);

impl AsFd for Card {
      fn as_fd(&self) -> BorrowedFd {
          self.0.as_fd()
      }
  }
impl Device for Card {} 
impl drm::Device for Card {}

impl Card {
    pub fn open(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        Card(file)
    }
}


