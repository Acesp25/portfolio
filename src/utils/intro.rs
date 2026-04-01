use maud::{Markup, html};

use crate::utils::images::AURA_IMG;

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout{
                .left-column {
                    h1 { "Hello! Im Aaron" }
                    h2 { "I do stuff sometimes"}   
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