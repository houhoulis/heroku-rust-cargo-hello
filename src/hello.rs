extern crate iron;
extern crate router;
extern crate logger;

use std::env;
use router::Router;
use iron::status;
use iron::prelude::*;
use logger::Logger;

mod all_out_of_bs;

// Serves a customized string to the user.  Try accessing "/world".
fn hello_name(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let name = params.find("name").unwrap();
    let resp = Response::with((status::Ok, format!("Hello, {}!", name)));
    Ok(resp)
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

/// Configure and run our server.
fn main() {
    println!("\n\nhi!!!! 👍🏿\n");

    // Set up our URL router.
    let mut router = Router::new();
    router.get("/", all_out_of_bs::bs, "index");
    router.get("/:name", hello_name, "name");

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);

    // Link logger_after as your *last* after middleware.
    chain.link_after(logger_after);

    // Run the server.
    Iron::new(chain).http(("0.0.0.0", get_server_port())).unwrap();
}
