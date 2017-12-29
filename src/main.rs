extern crate iron;
extern crate logger;
extern crate mount;
extern crate router;
extern crate staticfile;

use iron::prelude::*;
use iron::status;
use logger::Logger;
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;

fn main() {
    // Create our Router
    let router = {
        let mut router = Router::new();
        // Route "/" to handler with route_id "index"
        router.get("/", handler, "index");
        // Route "/:query" to handler with route_id "query"
        router.get("/:query", handler, "query");

        router
    };

    // Create our Mounts
    let mount = {
        let mut mount = Mount::new();
        // Bind Router to root
        mount.mount("/", router);
        // Except when overridden
        mount.mount("/html/", Static::new(Path::new("static/html/")));
        mount.mount("/css/", Static::new(Path::new("static/css/")));
        mount.mount("/js/", Static::new(Path::new("static/js/")));
        mount.mount("/img/", Static::new(Path::new("static/img/")));

        mount
    };

    // Create our Handler Chain
    let chain = {
        let mut chain = Chain::new(mount);
        // Create our loggers
        let (before, after) = Logger::new(None);
        // Link logger_before as your first before middleware.
        chain.link_before(before);
        // Link logger_after as your *last* after middleware.
        chain.link_after(after);

        chain
    };

    let listenaddr = format!(
        "0.0.0.0:{}",
        // Cloud Foundry & Docker provide a PORT
        match std::env::var("PORT") {
            Ok(port) => String::from(port),
            Err(_) => String::from("8080"),
        }
    );

    // Create our server
    let server = Iron::new(chain);

    // Start server
    let listener = server.http(listenaddr).unwrap();

    println!("{:?}", listener);
}

/// Simple Repeater Handler
fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}