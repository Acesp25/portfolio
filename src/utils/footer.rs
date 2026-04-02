use maud::{Markup, html};

pub fn footer() -> Markup {
    html! {
        .footer {
            p { "Be well" }
        }
    }
}