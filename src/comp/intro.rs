use maud::{Markup, html};

pub const HANDSOME_IMG: &str = "/public/images/me/introPFP.jpeg";

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout{
                .left-column {
                    .intro {
                        h1 { "Hello! Hola! 你好!" }
                        h2 { "I'm " strong { "Aaron Espinoza" } }
                        p { "I enjoy a good challenge and chocolate milk :)" }   
                        p { "My current interests include Operating Systems, Computer Networking, and Computer Vision." }
                    }
                }      
                .right-column {
                    .aura-container {
                        img alt="Image of my face" src=(HANDSOME_IMG);
                    }
                }
            }
        }
    }
}