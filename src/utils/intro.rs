use maud::{Markup, html};

pub const AURA_IMG: &str = "/public/images/me/aura.jpeg";

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout{
                .left-column {
                    h1 { "Hello! Bueno! 你好!" }
                    h2 { "I'm Aaron Espinoza" }
                    p { "I enjoy challenging myself and chocolate milk :)" }   
                    p { "My current interests include Operating Systems, Computer Networking, and Computer Vision." }
                }      
                .right-column {
                    .aura-container {
                        img alt="Image of me aurafarming" src=(AURA_IMG);
                    }
                }
            }
        }
    }
}