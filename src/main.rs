use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::{fs, io};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7070").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let uwubuffer = read_until_crlf(&mut stream);

        let response = router(uwubuffer);
        send_response(response, stream);
    }
}

fn read_until_crlf(stream: &mut TcpStream) -> String {
    let mut uwubuffer: String = String::new();
    //let mut buffer = [0; 1000];

    let mut buffread = BufReader::new(stream);
    buffread.read_line(&mut uwubuffer).unwrap();

    uwubuffer.pop();
    uwubuffer.pop();

    return uwubuffer;
}

fn router(uwubuffer: String) -> Response {
    return match uwubuffer.as_str() {
        "test1" => Response::Listing(Listing::new()),
        "test2" => Response::Listing(Listing::new()),
        "test3" => Response::Data(ItemType::BinaryFile, vec![9]),
        "" => Response::Listing(Listing::new()),
        _ => Response::Error,
    };
}

fn send_response(response: Response, mut stream: TcpStream) {
    let mut puffer: Vec<u8> = Vec::new();

    match response {
        Response::Listing(listings) => {
            for item in listings.items {
                match item.tuwu {
                    ItemType::BinaryFile => puffer.push(b'9'),
                    ItemType::TextFile => puffer.push(b'0'),
                    ItemType::Directory => puffer.push(b'1'),
                    ItemType::GIFFY => puffer.push(b'g'),
                    ItemType::IMGY => puffer.push(b'I'),
                }
                puffer.extend_from_slice(item.desc.as_bytes());
                puffer.push(9); //ASCII Tab Zeichen uwu :3
                puffer.extend_from_slice(item.path.as_bytes());
                puffer.push(9);
                puffer.extend_from_slice(item.server.as_bytes());
                puffer.push(9);
                puffer.extend_from_slice(item.port.to_string().as_bytes());
                //ASCII for CR LF uwu
                puffer.push(13);
                puffer.push(10);
            }
            puffer.push(b'.');
            puffer.push(13);
            puffer.push(10);
        }
        Response::Data(tuwu, mut data) => match tuwu {
            ItemType::TextFile => {
                puffer.append(&mut data);
                puffer.push(b'.');
                puffer.push(13);
                puffer.push(10);
            }
            _ => puffer.append(&mut data),
        },
        //TODO: Finish
        Response::Error => unimplemented!(),
    }

    stream.write_all(&puffer).unwrap();
}

fn read_files(uwupath: String) -> Vec<String> {
    let mut uwuentries = fs::read_dir(uwupath)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    uwuentries.sort();
    let uwuentries = uwuentries
        .into_iter()
        .map(|rest| rest.into_os_string().to_str().unwrap().to_string())
        .collect();
    return uwuentries;
}

fn create_directory_listing(uwuentries: Vec<String>) -> Vec<Item> {
    let mut uwuItems: Vec<Item> = Vec::new();
    let mut filetype = String::new();
    let md = metadata(filetype).unwrap();
    for x in uwuentries {
        if md.is_file() {
            let filetype = "0";
        } else if x.ends_with(".gif") {
            let filetype = "g";
        } else if x.ends_with(".jpg") {
            let filetype = "I";
        } else if md.is_dir() {
            let filetype = "1";
        };
    }
    return uwuItems;
}

enum Response {
    Listing(Listing),
    Data(ItemType, Vec<u8>),
    Error,
}

struct Listing {
    items: Vec<Item>,
}

impl Listing {
    fn new() -> Listing {
        let uwu: Vec<Item> = Vec::new();
        return Listing { items: uwu };
    }

    fn add(&mut self, x: Item) {
        self.items.push(x);
    }
}

enum ItemType {
    BinaryFile,
    TextFile,
    Directory,
    GIFFY,
    IMGY,
}
struct Item {
    tuwu: ItemType,
    desc: String,
    path: String,
    server: String,
    port: u32,
}
