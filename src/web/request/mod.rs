use std::net::TcpStream;
use std::io::Error;
use std::io::prelude::*;

pub mod request;

pub fn handle(request: Result<TcpStream,Error>) -> Option<request::Instance> 
{
    let out: Option<request::Instance>;
    out = match request {
        Ok(stream) => Some(consume(stream)),
        Err(err) => {println!("Error getting request: {}", err); return None;}
    };
    return out;
}

fn consume(mut stream: TcpStream) -> request::Instance {
    let mut buffer: Vec<u8> = vec![0; 1024];
    stream.read(&mut buffer).unwrap();
    let request: request::Instance = wrap_request(String::from_utf8_lossy(&buffer[..]).to_mut(), stream);
    return request;
}

fn wrap_request(inbound: &String, stream: TcpStream) -> request::Instance {
    let lines: Vec<String> = Vec::from_iter(inbound.split("\n").map(String::from));
    let request_params: Vec<String> = lines[0].split(' ').map(String::from).collect();
    println!("{} {}", request_params[0], request_params[1]);
    return request::Instance{
        path: request_params[1].to_owned(),
        status: 200,
        stream,
    };
}