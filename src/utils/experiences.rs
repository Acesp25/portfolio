use maud::{Markup, html};

const FREEBSD_LOGO: &str = "/public/images/experiences/fbsd.png";
const GPPHOSPITAL_LOGO: &str = "/public/images/experiences/gpph.png";
const TXST_LOGO: &str = "/public/images/experiences/txst.png";
const TEKRESCUE_LOGO: &str = "/public/images/experiences/tkrsc.png";

pub fn experiences() -> Markup {
    html! {
        .experience-container {
            (experience(
                "FreeBSD",
                "Google Summer of Code Student",
                "May 2025 - Sept 2025",
                FREEBSD_LOGO
            ))

            (experience(
                "Texas State University",
                "Deep Learning Researcher",
                "Aug 2024 - Present",
                TXST_LOGO
            ))

            (experience(
                "Guangdong People's Provincial Hospital",
                "Medical AI Research Intern",
                "May 2024 - Aug 2024",
                GPPHOSPITAL_LOGO
            ))

            (experience(
                "tekRESCUE",
                "Maintenance Technician",
                "Aug 2022 - May 2024",
                TEKRESCUE_LOGO
            ))
        }
    }
}

fn experience(name: &str, title: &str, date: &str, logo: &str) -> Markup {
    html! {
        .experience {
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
    }
}