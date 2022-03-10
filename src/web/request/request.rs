use std::net;
use std::io::prelude::*;

pub struct Instance {
    pub status: i16,
    pub path: String,
    pub stream: std::net::TcpStream
}

impl Instance {
    pub fn write(&mut self, content: String) {
        self.stream.write(content.as_bytes()).unwrap();
    }

    pub fn shutdown(&mut self) {
        self.stream.flush().unwrap();
    }
}