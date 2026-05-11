use std::fs;


pub fn list_monitors() {
    let cards = fs::read_dir("/dev/dri/").unwrap();
    for card_rs in cards {
        let card = card_rs.unwrap(); 
        let name = card.file_name();
    }
}
