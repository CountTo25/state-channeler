use std::net::TcpListener;
use std::env;

mod web;
mod app;
mod routes;
mod router;
mod controllers;

fn main() {
    let mut port: String = "5001".trim().to_string();
    let args: Vec<String> = env::args().collect();

    for arg in args {
        let split: Vec<String> = arg.split('=').map(String::from).collect();
        if split[0] != "--port" || split.len() != 2 {
            continue;
        }
        port = split[1].to_owned();
    }

    let address: String = format!("{}:{}", "0.0.0.0",port);
    println!("Booting at {}...", address);

    let socket = TcpListener::bind(address).unwrap();
    let app: app::App =  app::create();

    for request in socket.incoming() {
        app.session(
            web::request::handle(request).unwrap() //...more to come
        ).pipe();
    }
}
