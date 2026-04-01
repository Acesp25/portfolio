use rocket::get;
use maud::{html, Markup};

#[get("/")]
pub fn index() -> Markup {
    html! {
        h1 { "Hello, world!" }
        h2 { "blah blah blah"}
    }
}