use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs, io};
use std::net::{TcpListener, TcpStream};

mod gopher;

use crate::gopher::datatypes::*;
use crate::gopher::testing;
use crate::gopher::respuwing;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let uwubuffer = read_until_crlf(&mut stream);

        let response = router(uwubuffer);
        respuwing::send_response(response, stream);
    }
}

fn read_until_crlf(stream: &mut TcpStream) -> String {
    let mut uwubuffer: String = String::new();
    //let mut buffer = [0; 1000];

    let mut buffread = BufReader::new(stream);
    buffread.read_line(&mut uwubuffer).unwrap();
    println!("Last two bytes: {:?}", uwubuffer.chars().nth(uwubuffer.len() - 1).unwrap());
    // Remove \r\n
    // TODO: Dont asume the gender... Can be \n or \r\n. The testing Client uses only \n So we only pop once.
    uwubuffer.pop();
    //uwubuffer.pop();

    return uwubuffer;
}

fn router(uwubuffer: String) -> Response {
    println!("GOT REQUEST FOR: {}", uwubuffer.as_str());
    return match uwubuffer.as_str() {
        "" => Response::Listing(create_listing_from_dirpath("")),
        "../Cargo.toml" => Response::create_from_file( ItemType::TextFile ,"../Cargo.toml"),
        path => route_file_from_path(path),
    };
}

fn route_file_from_path(path: &str) -> Response {
    // TODO: Check if file exists.
    // TODO: Check File Type
    let mut full_path = "./root_dir/".to_string();
    full_path.push_str(path);

    match ItemType::get_item_type_from_path(full_path.as_str()) {
        Some(ItemType::Directory) => Response::Listing(create_listing_from_dirpath(path)),
        Some(item_type) => Response::create_from_file(item_type, full_path.as_str()),
        _ => Response::Error,
    }
    // TODO: send Response
}

/*fn create_directory_listing(uwuentries: Vec<String>) -> Vec<Item> {
    let mut uwuItems: Vec<Item> = Vec::new();
    let mut filetype = String::new();
    let md = metadata(filetype).unwrap();
    for x in uwuentries {
        if md.is_file() {
            let filetype = "0";
        } else if x.ends_with(".gif") {
            let filetype = "g";
        } else if x.ends_with(".png") {
            let filetype = "I";
        } else if md.is_dir() {
            let filetype = "1";
        };
    }
    return uwuItems;
}*/

fn create_listing_from_dirpath(path: &str) -> Listing {
    let mut full_path = "./root_dir/".to_string();
    full_path.push_str(path);

    let mut listing = Listing::new();

    for file in fs::read_dir(&full_path).unwrap() {
        match file {
            Ok(realfile) => {
                let response_path = realfile.path().to_str().unwrap().to_string();

                listing.add(
                    Item {
                        tuwu: ItemType::get_item_type_from_path(response_path.as_str()).unwrap(),
                        desc: realfile.file_name().into_string().unwrap(),
                        path: response_path.split("./root_dir/").nth(1).unwrap().to_string(),
                        server: "127.0.0.1".to_string(),
                        port: 7070,
                    }
                )
            }
            _ => {}
        }
    }
    println!("Listing: {:?}", listing);
    return listing;

}