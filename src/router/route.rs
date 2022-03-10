use crate::app::App;
use crate::web::request::request::Instance;
use crate::controllers::main::throw_404;
use crate::app::response::Response;

pub struct Route{
    pub method: RequestMethod,
    pub path: &'static str,
    pub callable: fn(&App, &Instance) -> Response,
    pub name: String,
}

impl Route {
    
}

impl Default for Route {
    fn default() -> Self {
        Self {
            method: RequestMethod::GET,
            path: "/",
            callable: throw_404,
            name: "404".to_string(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum RequestMethod {
    POST,
    GET,
}