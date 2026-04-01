use maud::{Markup, html};

use crate::utils::experiences::experiences;

pub fn body() -> Markup {
    html! {
        .section-container {
            input #exp-input type="radio" name="tab" checked;
            input #proj-input type="radio" name="tab";
            input #about-input type="radio" name="tab";
            input #read-input type="radio" name="tab";

            .tabs {
                label #exp-label for="exp-input" { "Experiences" }
                label #proj-label for="proj-input" { "Projects" }
                label #about-label for="about-input" { "About" }
                label #read-label for="read-input" { "Reading" }
            }
            .tab-content {
                #exp-content .tab {
                    (experiences())
                }
                #proj-content .tab {
                    p { "tab2 content" }
                }
                #about-content .tab {
                    p { "tab3 content" }
                }
                #read-content .tab {
                    p { "tab4 content" }
                }
            }
        }
    }
}