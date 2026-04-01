extern crate rocket;
extern crate maud;

use rocket::{launch, routes};

mod index;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index::index])
}