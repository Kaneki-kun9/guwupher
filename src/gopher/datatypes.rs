use std::vec::Vec;
use std::fs::{File, Metadata, metadata};
use std::path::Path;

use std::io::Read;

use mime_guess::{MimeGuess};
use mime_guess::mime;
use mime_guess::mime::Mime;

#[derive(Debug)]
pub enum ItemType {
    BinaryFile,
    TextFile,
    Directory,
    GIFFY,
    IMGY,
}

pub enum Response {
    Listing(Listing),
    Data(ItemType, Vec<u8>),
    Error,
}
#[derive(Debug)]
pub struct Listing {
    pub items: Vec<Item>,
}
#[derive(Debug)]
pub struct Item {
    pub tuwu: ItemType,
    pub desc: String,
    pub path: String,
    pub server: String,
    pub port: u32,
}


impl Listing {
    pub fn new() -> Listing {
        let uwu: Vec<Item> = Vec::new();
        return Listing { items: uwu };
    }

    pub fn add(&mut self, x: Item) {
        self.items.push(x);
    }
}   


impl Item {
    pub fn new(tuwu: ItemType) -> Item {
        Item {
            tuwu: tuwu,
            desc: "Test".to_string(),
            path: "test3".to_string(),
            server: "127.0.0.1".to_string(),
            port: 7070,
        }
    }
}

impl Response {
    pub fn create_from_file(memetype: ItemType, path: &str) -> Response {
        let mut file = File::open(path).unwrap();
        let mut datavec: Vec<u8> = Vec::new();
        file.read_to_end(&mut datavec).expect("IO Error while reading file");
        Response::Data {
            0: memetype,
            1: datavec,
        }
    }
}


impl ItemType {
    // Checks MIME type purely based on path.
    // Does not check whether file is valid, exists, or anything like that.
    pub fn get_item_type_from_path(path: &str) -> Option<ItemType> {
        // TODO: Check if File exists
        let path: &Path = Path::new(path);

        println!("Checking file {}", path.to_str().unwrap());

        if ! path.exists() {
            return None;
        }

        println!("File exists.");

        // TODO: File is directory
        if path.is_dir() {
            return Some(ItemType::Directory);
        }

        println!("File is not dir");

        if ! path.is_file() {
            return None;
        }

        println!("File is normal File");

        match MimeGuess::from_path(path).first() {
            Some(mime) => {
                println!("Got a Mime ({})",mime.type_());
                // match full mimes
                match mime.essence_str() {
                    "image/gif" => {return Some(ItemType::GIFFY);},
                    _ => {},
                }
                match mime.type_() {
                    mime::TEXT => Some(ItemType::TextFile),
                    mime::IMAGE => {println!("IMAGE!!!!"); Some(ItemType::IMGY)},
                    _ => Some(ItemType::BinaryFile),
                }
            },
            None => Some(ItemType::BinaryFile),
        }

    }
}