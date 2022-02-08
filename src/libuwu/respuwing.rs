use std::net::{TcpStream};
use std::io::Write;

use super::datatypes::*;

pub fn send_response(response: Response, mut stream: TcpStream) {
    let mut puffer: Vec<u8> = Vec::new();

    match response {
        Response::Listing(listings) => {
            println!("hiiiiii");
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
            println!("test....")
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
        Response::Error => {
            println!("unimplemented");
        }
    }

    stream.write_all(&puffer).unwrap();
}


