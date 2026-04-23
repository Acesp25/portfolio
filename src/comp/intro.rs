use maud::{Markup, html};

pub const HANDSOME_IMG: &str = "/public/images/me/introPFP.jpeg";

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout{
                .left-column {
                    .intro {
                        h1 { "Hello!" }
                        h2 { "I'm " strong { "Aaron Espinoza" } }
                        p { "I enjoy a good challenge and chocolate milk :)" }   
                        p { "My current interests include Operating Systems, Computer Networking, and Computer Vision." }
                        p { "Email: " a href="mailto:acesp25@freebsd.org"{"acesp25@FreeBSD.org"}}
                        p { "Github: " a href=("https://github.com/Acesp25") { "github/Acesp25" }}
                        p { "LinkedIn: " a href=("https://www.linkedin.com/in/aceespinoza") { "linkedin/aceespinoza" }}
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
