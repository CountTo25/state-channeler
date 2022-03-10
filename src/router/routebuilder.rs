use crate::router::route;
use crate::app::App;
use crate::web::request::request::Instance;
use crate::app::response::Response;

#[derive(Copy, Clone)]
pub struct RouteBuilder {
    pub m: route::RequestMethod,
    pub path: &'static str,
    pub name: &'static str,
}
impl RouteBuilder {
    pub fn name(&mut self, name: &'static str) -> Self {
        self.name = name;
        return self.chain();
    }

    fn chain(&self) -> Self {
        return *self
    }

    pub fn call(&self, callable:  fn(&App, &Instance) -> Response) -> route::Route {
        return route::Route{
            callable: callable,
            method: self.m,
            path: self.path,
            name: self.name.to_string(),
            ..Default::default()
        }
    }
}


impl Default for RouteBuilder {
    fn default() -> RouteBuilder {
        RouteBuilder {
            m: route::RequestMethod::GET,
            path: "/",
            name: "",
        }
    }
}