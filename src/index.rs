use rocket::get;
use maud::{Markup, html, DOCTYPE, PreEscaped};
use css_minify::optimizations::{Level, Minifier};

use crate::comp::intro::intro;
use crate::comp::body::body;
use crate::comp::footer::footer;

#[get("/")]
pub fn index() -> Markup {
    base (
        "Aaron Espinoza",
        "A Rust website about me :)",
        html! {
            (intro())
            (body())
            (footer())
        }
    )
}

fn base(name: &str, desc: &str, content: Markup) -> Markup {
	assert!(
		desc.len() <= 275,
		"desc too long per SERP limit"
	);

	html! {
		(DOCTYPE)

		html lang="en" {
			head {
				meta charset="UTF-8"; // must be in the first 1024 bytes of the document
				meta name="description" content=(desc);
                title { (name) }

                link rel="icon" type="image/ico" href="/public/images/me/aura.ico";

				style { 
                    (include_css!("/main.css"))
                    (include_css!("/base.css"))
                    (include_css!("/body.css"))
                }
            }

            body {
                (content)
            }
        }
    }
}