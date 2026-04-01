use rocket::get;
use maud::{Markup, html};

use crate::base::base;

use crate::utils::intro::intro;
use crate::utils::body::body;

#[get("/")]
pub fn index() -> Markup {
    base (
        "Aaron's Portfolio",
        "A Rust website about me :)",
        html! {
            (intro())
            (body())
        }
    )
}