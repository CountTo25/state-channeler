use crate::app::App;
use crate::web::request::request::Instance as Request;
use crate::app::response::Response;

pub fn say_hello(_app: &App, _request: &Request) -> Response {
    println!("Nice but why do we get here");
    Response {
        status: 200,
        body: "Works".to_string(), //TODO: figure out how to make this elegant
        ..Default::default()
    }
}

pub fn wow(_app: &App, _request: &Request) -> Response {
    println!("even cooler");
    Response {
        status: 200,
        body: "Some other URL".to_string(), //TODO: figure out how to make this elegant
        ..Default::default()
    }
}

pub fn throw_404(_app: &App, _request: &Request) -> Response {
    println!("404");
    Response {
        status: 404,
        body: "Not found".to_string(), //TODO: figure out how to make this elegant
        ..Default::default()
    }
}