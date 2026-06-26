use maud::{Markup, html};

pub const HANDSOME_IMG:     &str = "/public/images/me/introPFP.jpeg";

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout{
                .left-column {
                    .intro {
                        h1 { "Hello!" }
                        h2 { "I'm " strong { "Aaron Espinoza" } }
                        p { "A computer programmer and chocolate milk fanatic :)" }
                        p { "My current interests include Computer Networking, Operating Systems, and Computer Vision." }
                        p { "Graduating December 2026, looking for new grad roles in systems/kernel engineering or applied CV/Ml." }
                        p { "Email: " a href="mailto:me@acespinoza.net"{"me@acespinoza.net"}}
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
