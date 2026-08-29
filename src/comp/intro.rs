use maud::{Markup, html};

use super::images::{Loading, Photo};

pub const INTRO_PFP: Photo = Photo::me("introPFP", "Aaron Espinoza");

pub fn intro() -> Markup {
    html! {
        .section-container {
            .split-layout {
                .left-column {
                    .intro {
                        h1 { "Hello!" }
                        h2 { "I'm " strong { "Aaron Espinoza" } }
                        p { "A computer programmer and chocolate milk fanatic :)" }
                        p {
                            "My current interests include computer networking, operating "
                            "systems, and computer vision."
                        }
                        p {
                            "Graduating December 2026, looking for new grad roles in "
                            "systems/kernel engineering or applied CV/ML."
                        }
                        p { "Email: " a href="mailto:me@acespinoza.net" { "me@acespinoza.net" } }
                        p { "GitHub: " a href="https://github.com/Acesp25" { "github/Acesp25" } }
                        p {
                            "LinkedIn: "
                            a href="https://www.linkedin.com/in/aceespinoza" { "linkedin/aceespinoza" }
                        }
                    }
                }
                .right-column {
                    .aura-container {
                        /* Above the fold, we use Eager here */
                        (INTRO_PFP.img(Loading::Eager))
                    }
                }
            }
        }
    }
}
