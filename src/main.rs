use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

const HOST: &str = "127.0.0.1";
const PORT: u32 = 8080;

fn main() {
    let address = format!("{}:{}", HOST, PORT);

    let server = TcpListener::bind(&address).unwrap();
    println!("Listening on {address}");
    
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).expect("Failed to accept websocket");
            websocket.write_message("Welcome to mup!d up!".into()).expect("Failed to write message");

            loop {
                let msg = match websocket.read_message() {
                    Ok(msg) => msg,
                    Err(err) => match err {
                        tungstenite::Error::ConnectionClosed => {
                            println!("Connection closed");
                            break;
                        }
                        _ => {
                            println!("Error: {}", err);
                            break;
                        }
                    },
                };

                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).expect("Failed to write message");
                }
            }
        });
    }
}
