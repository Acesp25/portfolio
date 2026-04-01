use maud::{Markup, html};

const FREEBSD_LOGO: &str = "/public/images/experiences/fbsd.png";
const GPPHOSPITAL_LOGO: &str = "/public/images/experiences/gpph.png";
const TXST_LOGO: &str = "/public/images/experiences/txst.png";
const TEKRESCUE_LOGO: &str = "/public/images/experiences/tkrsc.png";

pub fn experiences() -> Markup {
    html! {
        .experience-container {
            (experience("FreeBSD", "Google Summer of Code Student", FREEBSD_LOGO))
            (experience("Texas State University", "Deep Learning Researcher", TXST_LOGO))
            (experience("GuangDong People's Provincial Hospital", "Medical AI Research Intern", GPPHOSPITAL_LOGO))
            (experience("TekRescue", "Maintenance Technician", TEKRESCUE_LOGO))
        }
    }
}

fn experience(name: &str, title: &str, logo: &str) -> Markup {
    html! {
        .experience {
            .logo-container {
                img alt=(name) src=(logo);
            }
            .info-container {
                h3 { (name) }
                p { (title) }
            }
        }
    }
}