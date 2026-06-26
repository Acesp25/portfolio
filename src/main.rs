extern crate css_minify;
extern crate maud;
extern crate rocket;

use rocket::{fs::FileServer, launch, routes};

#[macro_use]
mod macros;

mod comp;
mod index;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index])
        .mount("/public", FileServer::from(relative!("/public")))
}

