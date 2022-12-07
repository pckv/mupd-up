use std::{net::TcpStream};
use tungstenite::WebSocket;

use super::types::{Packet, Palette};

pub trait WebSocketExt<T> {
    fn write_api_packet(&mut self, packet: Packet) -> Result<(), tungstenite::Error>;
    fn write_api_message(&mut self, message: String) -> Result<(), tungstenite::Error>;
    fn write_api_palette(&mut self, palette: Palette) -> Result<(), tungstenite::Error>;
}

impl WebSocketExt<TcpStream> for WebSocket<TcpStream> {
    fn write_api_packet(&mut self, packet: Packet) -> Result<(), tungstenite::Error> {
        self.write_message(serde_json::to_string(&packet).unwrap().into())
    }

    fn write_api_message(&mut self, message: String) -> Result<(), tungstenite::Error> {
        self.write_api_packet(Packet { message: Some(message), palette: None })
    }

    fn write_api_palette(&mut self, palette: Palette) -> Result<(), tungstenite::Error> {
        self.write_api_packet(Packet { message: None, palette: Some(palette) })
    }
}