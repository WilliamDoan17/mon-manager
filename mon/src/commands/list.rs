use std::fs;
use std::fs::DirEntry;
use crate::drm::Card;
use crate::types::monitor::{Monitor};

fn open_card_file(card_file: DirEntry) -> Option<Card> {
        let name = card_file.file_name();
    let name_str = name.to_str().unwrap();
    if !name_str.starts_with("card") {
        return None
    }
    let path = card_file.path();
    let path_str = path.to_str().unwrap();
    let card : Card = Card::open(path_str); 
    Some(card)
}


pub fn list_monitors() {
    let cards = fs::read_dir("/dev/dri/").unwrap();
    let mut monitors : Vec<Monitor>; 
    for card_rs in cards {
        let card_file = card_rs.unwrap();
        if let Some(card) = open_card_file(card_file) {

        }
    }
}
