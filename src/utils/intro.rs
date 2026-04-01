use maud::{Markup, html};

use crate::utils::images::AURA_IMG;

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
                        img src=(AURA_IMG);
                    }
                }
            }
        }
    }
}