use maud::{Markup, html};

const FREEBSD_LOGO: &str = "/public/images/experiences/fbsd.png";
const GPPHOSPITAL_LOGO: &str = "/public/images/experiences/gpph.png";
const TXST_LOGO: &str = "/public/images/experiences/txst.png";
const TEKRESCUE_LOGO: &str = "/public/images/experiences/tkrsc.png";

pub fn experiences() -> Markup {
    html! {
        .body-container {
            (experience(
                1,
                "FreeBSD",
                "Google Summer of Code Student",
                "May 2025 - Sept 2025",
                FREEBSD_LOGO,
                html! {
                    p { "test lalalala"}
                }
            ))

            (experience(
                2,
                "Texas State University",
                "Deep Learning Researcher",
                "Aug 2024 - Present",
                TXST_LOGO,
                html! {
                    p { "test lalalala"}
                }
            ))

            (experience(
                3,
                "Guangdong People's Provincial Hospital",
                "Medical AI Research Intern",
                "May 2024 - Aug 2024",
                GPPHOSPITAL_LOGO,
                html! {
                    p { "test lalalala"}
                }
            ))

            (experience(
                4,
                "tekRESCUE",
                "Maintenance Technician",
                "Aug 2022 - May 2024",
                TEKRESCUE_LOGO,
                html! {
                   p { "test lalalala"} 
                }
            ))
        }
    }
}

fn experience(id: i32, name: &str, title: &str, date: &str, logo: &str, info: Markup) -> Markup {
    let toggle_id = format!("modal-{}", id);
    html! {
        input type="checkbox" id=(toggle_id) class="modal-state";

        label for=(toggle_id) class="experience" {
            .logo-container {
                img alt=(name) src=(logo);
            }
            .info-container {
                .header-row {
                    h3 { (name) }
                    p.date { (date) }
                }
                p { (title) }
            }
        }

        div class="modal-overlay" {
            label for=(toggle_id) class="modal-backdrop" {};
            
            div class="modal-content" {
                label for=(toggle_id) class="close-btn" { "x" }

                .modal-header {
                    img src=(logo) alt=(name);
                    h3 { (title) }
                    p.date { (date) }
                }
                .modal-details { 
                    (info)
                }
            }
        }
    }
}