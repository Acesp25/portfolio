use maud::{Markup, html, DOCTYPE, PreEscaped};
use css_minify::optimizations::{Level, Minifier};

pub fn base(name: &str, desc: &str, content: Markup) -> Markup {
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

				style { (include_css!("/main.css")) }
            }

            body {
                (content)
            }
        }
    }
}