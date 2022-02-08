use std::vec::Vec;
use std::fs::{File};
use std::path::Path;

use std::io::Read;

use mime_guess::{MimeGuess};
use mime_guess::mime;



#[derive(Debug)]
pub enum ItemType {
    BinaryFile,
    TextFile,
    Directory,
    GIFFY,
    IMGY,
}

#[derive(Debug)]
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
        let path: &Path = Path::new(path);

        println!("Checking file {}", path.to_str().unwrap());

        if ! path.exists() {
            println!("is this our destiny?");
            return None;
        }

        println!("File exists.");

        if path.is_dir() {
            return Some(ItemType::Directory);
        }

        println!("File is not dir");

        if ! path.is_file() {
            println!("are we in hell?");
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