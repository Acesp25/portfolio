use maud::{Markup, html};

pub fn body() -> Markup {
    html! {
        .section-container {
            input #tab1-input type="radio" name="tab" checked;
            input #tab2-input type="radio" name="tab";
            .tabs {
                label #tab1-label for="tab1-input" { "tab1" }
                label #tab2-label for="tab2-input" { "tab2" }
            }
            .tab-content {
                #tab1-content .tab {
                    p { "tab1 content" }
                }
                #tab2-content .tab {
                    p { "tab2 content" }
                }
            }
        }
    }
}