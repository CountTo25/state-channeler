mod routebuilder;
mod route;


pub struct Router {
    pub routes: Vec<route::Route>,
}

impl Router {
    pub fn find(&self,path: String) -> &route::Route {
        let opt: Option<&route::Route> = self.routes.iter().filter(|r| r.path == path).next(); //for now
        match opt {
            Some(r) => r,
            None => &self.named("404".to_string()),
        }
    }

    pub fn named(&self, name: String) -> &route::Route {
       return self.routes.iter().filter(|r| r.name == name).next().unwrap(); //for now
    }
}

pub fn write(routes: Vec<route::Route>) -> Router {
    return Router{routes: routes}
}

pub fn get(path: &'static str) -> routebuilder::RouteBuilder 
{
    return routebuilder::RouteBuilder{m: route::RequestMethod::GET, path, ..Default::default()}
}
pub fn post(path: &'static str) -> routebuilder::RouteBuilder 
{
    return routebuilder::RouteBuilder{m: route::RequestMethod::POST, path, ..Default::default()}
}