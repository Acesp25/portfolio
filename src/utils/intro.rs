use maud::{Markup, html};

pub const AURA_IMG: &str = "/public/images/aura.jpeg";

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout{
                .left-column {
                    h1 { "Welcome to my portfolio" }
                    h2 { "Blah Blah BLah I do stuff sometimes"}   
                }      
                .right-column {
                    .aura-container {
                        img src=(AURA_IMG);
                    }
                }
            }
        }
    }
}