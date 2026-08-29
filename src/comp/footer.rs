use maud::{Markup, html};

pub fn footer() -> Markup {
    html! {
        .section-container {
            .footer {
                p { strong { "Be well" } }
            }
        }
    }
}
