use maud::{Markup, html};

use crate::comp::about::about;
use crate::comp::experiences::experiences;
use crate::comp::projects::projects;
use crate::comp::reading::reading;

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
                    (projects())
                }
                #about-content .tab {
                    (about())
                }
                #read-content .tab {
                    (reading())
                }
            }
        }
    }
}

