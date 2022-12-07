use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

mod api;
use api::{types::{Packet}, messaging::WebSocketExt};

const HOST: &str = "127.0.0.1";
const PORT: u32 = 8080;

fn main() {
    let address = format!("{}:{}", HOST, PORT);

    let server = TcpListener::bind(&address).unwrap();
    println!("Listening on {address}");
    
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).expect("Failed to accept websocket");
            let address = websocket.get_mut().peer_addr().unwrap();
            println!("{:?}: Connected", address);

            websocket.write_api_packet(Packet {
                message: Some("Welcome to &xmup!d up!".to_owned()),
                palette: Some(api::palettes::MAIN_PALETTE),
            }).expect("Failed to write message");

            loop {
                let msg = match websocket.read_message() {
                    Ok(msg) => msg,
                    Err(err) => match err {
                        tungstenite::Error::ConnectionClosed => {
                            println!("{:?}: Disconnected", address);
                            break;
                        }
                        _ => {
                            println!("Error: {:?}", err);
                            break;
                        }
                    },
                };

                let content = msg.to_text().unwrap();

                if msg.is_binary() || msg.is_text() {
                    websocket.write_api_message(content.to_owned()).expect("Failed to write message");
                }
            }
        });
    }
}
