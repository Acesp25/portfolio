use maud::{Markup, html};
use std::sync::LazyLock;

const FREEBSD_LOGO:     &str = "fbsd.png";
const GPPHOSPITAL_LOGO: &str = "gpph.png";
const TXST_LOGO:        &str = "txst.png";
const TEKRESCUE_LOGO:   &str = "tkrsc.png";

pub fn experiences() -> Markup {
    html! {
        .body-container {
            ((*FREEBSD_EXP).showcase())
            ((*TXST_EXP).showcase())
            ((*FREEBSD_GSOC_EXP).showcase())
            ((*GPPH_EXP).showcase())
            ((*TECH_EXP).showcase())
        }
    }
}

static FREEBSD_EXP: LazyLock<Experience<'static>> = LazyLock::new(|| Experience {
    name:   "FreeBSD",
    title:  "Open Source Contributor",
    date:   "Sept 2025 - Present",
    logo:   FREEBSD_LOGO,
    desc: {
        html! {
            p {
                "Main interest is network drivers, with additional work on GPIO and character devices. "
                "Currently developing a veb(4) variant for FreeBSD. "
                "Landed a cleanup in if_bridge(4) removing dead code."
            }
        }
    },
});

static TXST_EXP: LazyLock<Experience<'static>> = LazyLock::new(|| Experience {
    name:   "Texas State University",
    title:  "Deep Learning Researcher",
    date:   "Aug 2024 - Present",
    logo:   TXST_LOGO,
    desc: {
        html! {
            p {
                "Conducting research on deep learning techniques for GAN-based adversarial perturbation generation. "
                "The project focuses on utilizing hybrid UNet models to create effective and discreet adversarial perturbations that can perform impersonation attacks on facial recognition systems. "
            }
            p {
                "Paper under review for AAAI 2027"
            }
        }
    },
});

static FREEBSD_GSOC_EXP: LazyLock<Experience<'static>> = LazyLock::new(|| Experience {
    name:   "FreeBSD",
    title:  "Google Summer of Code Student",
    date:   "May 2025 - Sept 2025",
    logo:   FREEBSD_LOGO,
    desc: {
        html! {
            p {
                "Project focused on the testing and development for FreeBSD Rust kernel device drivers. "
                "The outcome was a modular Rust framework for developing Rust based FreeBSD kernel device drivers, along with a testing suite to ensure reliability and performance. "
            }
            p {
                "Writeup: " a href="https://gist.github.com/Acesp25/8928e35e710fdce1896b5448fc6327df" { "Acesp25/gsoc2025" }
            }
            p {
                "Official FreeBSD video: " a href="https://youtu.be/y82-t1tDLWg?si=7ad9fqQwQmaaZdQp" { "FreeBSD GSoC 2025 - Aaron Espinoza" }
            }
        }
    },
});

static GPPH_EXP: LazyLock<Experience<'static>> = LazyLock::new(|| Experience {
    name:   "Guangdong Provincial People's Hospital",
    title:  "Medical AI Research Intern",
    date:   "May 2024 - Aug 2024",
    logo:   GPPHOSPITAL_LOGO,
    desc: {
        html! {
            p {
                "Won 1st place in the 2024 MICCAI WHS++ Challenge. "
                "Created ensembled deep learning models for accurate whole heart segmentation in CT and MRI scans. "
            }
            p {
                "MICCAI publication: " a href="https://dl.acm.org/doi/10.1007/978-3-031-87009-5_13" { "ACM DL Link" }
            }
        }
    },
});

static TECH_EXP: LazyLock<Experience<'static>> = LazyLock::new(|| Experience {
    name:   "tekRESCUE",
    title:  "Maintenance Technician",
    date:   "Aug 2022 - May 2024",
    logo:   TEKRESCUE_LOGO,
    desc: {
        html! {
            p {
                "I was responsible for performing weekly maintenance on a variety of computer systems for both large and small clients. "
                "I was also occasionally tasked with customizing servers and performing on-site troubleshooting for clients. "
            }
        }
    },
});

struct Experience<'a> {
    name:   &'a str,
    title:  &'a str,
    date:   &'a str,
    logo:   &'a str,
    desc:   Markup,
}
impl<'a> Experience<'a> {
    fn showcase(&self) -> Markup {
        let dirty = format!("exp-{}-{}", self.name, self.title);
        let toggle_id = dirty
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        let logo_img = format!("/public/images/experiences/{}", self.logo);

        html! {
            input type="checkbox" id=(toggle_id) class="modal-state";

            label for=(toggle_id) class="experience" {
                .logo-container {
                    img src=(logo_img) alt=(self.name);
                }
                .info-container {
                    .header-row {
                        h3 { (self.name) }
                        p.date { (self.date) }
                    }
                    p { (self.title) }
                }
            }

            .modal-overlay {
                label for=(toggle_id) class="modal-backdrop" {};

                .exp-modal-content {
                    label for=(toggle_id) class="close-btn" { "x" }

                    .exp-modal-header {
                        img src=(logo_img) alt=(self.name);
                        h3 { (self.title) }
                        p.date { (self.date) }
                    }
                    .exp-modal-details {
                        (self.desc)
                    }
                }
            }
        }
    }
}
