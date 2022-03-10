use crate::web::request::request::Instance;
use crate::router::Router;

pub struct Session<'a> {
    request: Instance,
    app: &'a App,
}

pub mod response;

impl Session<'_> { //TODO: need to learn how this works
    pub fn get_request(&self) -> &Instance {
        return &self.request
    }

    fn collapse(&self) -> () { //pointless? Maybe
        drop(self);
    }

    pub fn pipe(&mut self) -> () {
        let router = self.app.router.as_ref().unwrap();
        let route = router.find(self.request.path.to_owned());
        let response: response::Response = (route.callable)(self.app, &mut self.request);
        self.request.write(response.body);
        self.request.shutdown();
        self.collapse(); //TODO learn about releasing memory from useless instances
    }
    
}

pub struct App{
    router: Option<Router>
}

impl App {

    pub fn session(&self,request: Instance) -> Session {
        return Session{request, app: self}
    }

    pub fn bind_routes(&mut self, routes: Router) -> &App {
        self.router = Some(routes);
        return self;
    }
}


pub fn create() -> App {
    return App{router: Some(crate::routes::register())};
}


trait Collapsable {
    fn collapse(&self) -> ();
}

trait Container {
    fn container(&self) -> ();
}

trait Request {
    fn request(&self) -> Instance;
}

impl Container for App {
    fn container(&self) -> () {

    }
}