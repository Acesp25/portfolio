use maud::{html, Markup};

use crate::utils::intro::intro;

pub fn content() -> Markup {
    html! {
        .intro { (intro()) }
    }
}