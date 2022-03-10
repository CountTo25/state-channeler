use crate::router;
use crate::controllers;

pub fn register() -> router::Router {
    return router::write(vec![
        router::get("/").call(controllers::main::say_hello),
        router::get("/wow").call(controllers::main::wow),
        router::get("/err").name("404").call(controllers::main::throw_404)
    ])
}