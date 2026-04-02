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
                    p {
                        "Project focused on the testing and development for FreeBSD Rust kernel device drivers. "
                        "The outcome was a modular Rust framework for developing Rust based FreeBSD kernel device drivers, along with a testing suite to ensure reliability and performance. "
                    }                    
                    p {
                        "Detailed writeup: " a href="https://gist.github.com/Acesp25/8928e35e710fdce1896b5448fc6327df" { "Acesp25/gsoc2025" }
                    }
                    p {
                        "Official FreeBSD video: " a href="https://youtu.be/y82-t1tDLWg?si=7ad9fqQwQmaaZdQp" { "FreeBSD GSoC 2025 - Aaron Espinoza" }
                    }
                }
            ))

            (experience(
                2,
                "Texas State University",
                "Deep Learning Researcher",
                "Aug 2024 - Present",
                TXST_LOGO,
                html! {
                    p {
                        "Conducting research on deep learning techniques for GAN-based adversarial perturbation generation. "
                        "The project focuses on utililzing modern Unet techniques to create effective and discreet adversarial perturbations that can preform obfusacion and impersonation attacks on facial recognition systems. "
                    }
                    p {
                        "Paper pending submission."
                    }
                }
            ))

            (experience(
                3,
                "Guangdong People's Provincial Hospital",
                "Medical AI Research Intern",
                "May 2024 - Aug 2024",
                GPPHOSPITAL_LOGO,
                html! {
                    p { 
                        "Won 1st place in the 2024 MICCAI WHS++ Challenge. "
                        "Created ensembled deep learning models for accurate whole heart segmentation in CT and MRI scans. "
                    }
                    p {
                        "MICCAI publication: " a href="https://dl.acm.org/doi/10.1007/978-3-031-87009-5_13" { "ACM DL Link" }
                    }
                }
            ))

            (experience(
                4,
                "tekRESCUE",
                "Maintenance Technician",
                "Aug 2022 - May 2024",
                TEKRESCUE_LOGO,
                html! {
                   p {
                        "I was responsible for preforming weekly maintenance on a variety of computer systems for both large and small clients. "
                        "I was also occationally tasked with customizing severs and preforming on-sight troubleshooting. "
                   } 
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