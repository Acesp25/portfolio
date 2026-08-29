use maud::{Markup, html};
use std::sync::LazyLock;

use super::images::{Loading, Photo};
use super::ui::{Link, Modal, link_list, slug};

pub fn experiences() -> Markup {
    html! {
        .body-container {
            (FREEBSD_OSS.showcase())
            (TXST.showcase())
            (FREEBSD_GSOC.showcase())
            (GPPH.showcase())
            (TEKRESCUE.showcase())
        }
    }
}

const FREEBSD_LOGO: Photo = Photo::experience("fbsd", "FreeBSD");
const TXST_LOGO: Photo = Photo::experience("txst", "Texas State University");
const GPPH_LOGO: Photo = Photo::experience("gpph", "Guangdong Provincial People's Hospital");
const TEKRESCUE_LOGO: Photo = Photo::experience("tkrsc", "tekRESCUE");

static FREEBSD_OSS: LazyLock<Experience> = LazyLock::new(|| Experience {
    name: "FreeBSD",
    title: "Open Source Contributor",
    date: "Sept 2025 - Present",
    logo: FREEBSD_LOGO,
    desc: html! {
        p {
            "Main interest is network drivers, with additional work on GPIO and character "
            "devices. Currently developing a veb(4) variant for FreeBSD. Landed a cleanup "
            "in if_bridge(4) removing dead code."
        }
    },
    links: vec![],
});

static TXST: LazyLock<Experience> = LazyLock::new(|| Experience {
    name: "Texas State University",
    title: "Deep Learning Researcher",
    date: "Aug 2024 - Present",
    logo: TXST_LOGO,
    desc: html! {
        p {
            "Research on deep learning techniques for GAN-based adversarial perturbation "
            "generation. The project uses hybrid UNet models to produce effective and "
            "imperceptible perturbations that carry out impersonation attacks against "
            "facial recognition systems."
        }
        p { "Paper under review for AAAI 2027." }
    },
    links: vec![],
});

static FREEBSD_GSOC: LazyLock<Experience> = LazyLock::new(|| Experience {
    name: "FreeBSD",
    title: "Google Summer of Code Student",
    date: "May 2025 - Sept 2025",
    logo: FREEBSD_LOGO,
    desc: html! {
        p {
            "Testing and development for FreeBSD Rust kernel device drivers. The outcome "
            "was a modular Rust framework for building FreeBSD kernel device drivers in "
            "Rust, plus a test suite covering reliability and performance."
        }
    },
    links: vec![
        Link {
            label: "Writeup: Acesp25/gsoc2025",
            href: "https://gist.github.com/Acesp25/8928e35e710fdce1896b5448fc6327df",
        },
        Link {
            label: "FreeBSD GSoC 2025 showcase",
            href: "https://youtu.be/y82-t1tDLWg",
        },
    ],
});

static GPPH: LazyLock<Experience> = LazyLock::new(|| Experience {
    name: "Guangdong Provincial People's Hospital",
    title: "Medical AI Research Intern",
    date: "May 2024 - Aug 2024",
    logo: GPPH_LOGO,
    desc: html! {
        p {
            "Placed 1st in the 2024 MICCAI WHS++ Challenge. Built ensembled deep learning "
            "models for whole-heart segmentation in CT and MRI scans."
        }
    },
    links: vec![Link {
        label: "MICCAI publication (ACM DL)",
        href: "https://dl.acm.org/doi/10.1007/978-3-031-87009-5_13",
    }],
});

static TEKRESCUE: LazyLock<Experience> = LazyLock::new(|| Experience {
    name: "tekRESCUE",
    title: "Maintenance Technician",
    date: "Aug 2022 - May 2024",
    logo: TEKRESCUE_LOGO,
    desc: html! {
        p {
            "Performed weekly maintenance on a range of computer systems for both large "
            "and small clients, and was occasionally tasked with customizing servers and "
            "on-site troubleshooting."
        }
    },
    links: vec![],
});

struct Experience {
    name:   &'static str,
    title:  &'static str,
    date:   &'static str,
    logo:   Photo,
    desc:   Markup,
    links:  Vec<Link>,
}

impl Experience {
    fn showcase(&self) -> Markup {
        Modal {
            id: slug(&["exp", self.name, self.title]),
            card_class: "experience",
            card: html! {
                (self.logo.framed(Loading::Lazy))
                .info-container {
                    .header-row {
                        h3 { (self.name) }
                        p.date { (self.date) }
                    }
                    p { (self.title) }
                }
            },
            panel_class: "exp-modal-content",
            panel: html! {
                .exp-modal-header {
                    /*
                     * Decorative: the employer name is already in the heading
                     * beside it, so announcing the logo repeats it.
                     */
                    (self.logo.decorative(Loading::Lazy))
                    h3 { (self.title) }
                    p.date { (self.date) }
                }
                .exp-modal-details {
                    (self.desc)
                    (link_list(&self.links))
                }
            },
        }
        .render()
    }
}
