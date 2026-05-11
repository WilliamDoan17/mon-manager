use std::fs::OpenOptions;
use drm::control::Device;

pub struct Card(std::fs::File);

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


