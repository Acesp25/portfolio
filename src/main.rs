extern crate rocket;
extern crate maud;
extern crate css_minify;

use rocket::{launch, routes};

#[macro_use]
mod macros;

mod index;
mod base;
mod body;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index::index])
}