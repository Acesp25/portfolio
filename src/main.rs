extern crate rocket;
extern crate maud;
extern crate css_minify;

use rocket::{launch, routes, fs::FileServer};

#[macro_use]
mod macros;

mod utils;
mod index;
mod base;
mod content;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index])
        .mount("/public", FileServer::from(relative!("/public")))
}