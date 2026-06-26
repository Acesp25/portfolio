use css_minify::optimizations::{Level, Minifier};
use maud::{DOCTYPE, Markup, PreEscaped, html};
use rocket::get;

use crate::comp::body::body;
use crate::comp::footer::footer;
use crate::comp::intro::intro;

#[get("/")]
pub fn index() -> Markup {
    base(
        "Aaron Espinoza",
        "A website about me :)",
        html! {
            (intro())
            (body())
            (footer())
        },
    )
}

const SITE_URL: &str = "https://acespinoza.net";

fn base(name: &str, desc: &str, content: Markup) -> Markup {
    assert!(desc.len() <= 275, "desc too long per SERP limit");

    html! {
        (DOCTYPE)

        html lang="en" {
            head {
                meta charset="UTF-8"; // must be in the first 1024 bytes of the document
                meta name="description" content=(desc);
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (name) }

                meta property="og:type" content="website";
                meta property="og:site_name" content="Aaron Espinoza";
                meta property="og:title" content=(name);
                meta property="og:description" content=(desc);
                meta property="og:url" content=(SITE_URL);

                link rel="icon" type="image/ico" href="/public/images/me/aura.ico";

                style {
                    (include_css!("/main.css"))
                    (include_css!("/base.css"))
                    (include_css!("/body.css"))
                    (include_css!("/mobile.css"))
                }
            }

            body {
                (content)
            }
        }
    }
}

