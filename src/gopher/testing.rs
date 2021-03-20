use crate::SERVER_ADRESS;

use super::datatypes::*;

pub fn create_test_image() -> Item {
    Item {
        tuwu: ItemType::IMGY,
        desc: "TestIMG".to_string(),
        path: "test.png".to_string(),
        server: SERVER_ADRESS.to_string(),
        port: 7070,
    }
}

pub fn create_test_text() -> Item {
    Item {
        tuwu: ItemType::TextFile,
        desc: "TestTxt".to_string(),
        path: "test.txt".to_string(),
        server: SERVER_ADRESS.to_string(),
        port: 7070,
    }
}

pub fn create_test_listing() -> Listing {
    let mut listing = Listing::new();
    listing.add(create_test_image());
    listing.add(create_test_text());
    
    return listing;
}