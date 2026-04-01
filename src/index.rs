use rocket::get;
use maud::Markup;

use crate::base::base;
use crate::content::content;

#[get("/")]
pub fn index() -> Markup {
    base (
        "Aaron's Portfolio",
        "A Rust website about me :)",
        content()
    )
}