use maud::{Markup, html};
use std::sync::LazyLock;

const RUSTKLD_IMG:  &str = "rustkld.png";
const VEB_IMG:      &str = "veb.png";
const VTGPIO_IMG:   &str = "vtgpio.png";
const TASTE_IMG:    &str = "tastebase.png";
const SETUP_IMG:    &str = "setup.jpg";
const PINGPONG_IMG: &str = "pingpong.jpg";
const WEBSITE_IMG:  &str = "website.png";

pub fn projects() -> Markup {
    html! {
        .body-container {
            ((*RUSTKLD_PROJ).showcase())
            ((*VEB_PROJ).showcase())
            ((*VTGPIO_PROJ).showcase())
            ((*TASTEBASE_PROJ).showcase())
            ((*PINGPONG_PROJ).showcase())
            ((*SETUP_PROJ).showcase())
            ((*WEBSITE_PROJ).showcase())
        }
    }
}

static RUSTKLD_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "RustKLD",
    brief:  "FreeBSD Framework for making Rust Kernel Device Drivers",
    img:    RUSTKLD_IMG,
    desc: {
        html! {
            p {
                "This framework was a result from my GSoC project with FreeBSD. "
                "It allows developers to easily create their own Rust device drivers for FreeBSD. "
                "Includes a modular, expandable structure with CI integration and "
                "documentation to further assist developers wanting to learn and work with it. "
            }
            p { b { "Code Link: "} a href=("https://github.com/Acesp25/rustkld") { "Acesp/rustkld" } }
        }
    },
});

static VEB_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "Veb Network Driver (WIP)",
    brief:  "Virtual Ethernet Bridge network pseudo-device for FreeBSD",
    img:    VEB_IMG,
    desc: {
        html! {
            p {
                "This FreeBSD network driver was inspired by OpenBSD's veb(4). "
                "It's structurally similar to bridge(4) but is a stripped-down variant for Jails and Bhyve VMs. "
                "This driver also hides itself from the host-network stack. These features allow for "
                "faster network connections, perfect for latency-dependent use-cases."
            }
        }
    },
});

static VTGPIO_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "VirtIO GPIO Driver (WIP)",
    brief:  "VirtIO General Input/Output Driver for FreeBSD",
    img:    VTGPIO_IMG,
    desc: {
        html! {
            p {
                "This project was to familiarize myself with more technical device drivers for FreeBSD. "
                "It was designed using the Version 1.3 documentation provided by Oasis-Open. "
                "I originally wrote it in C, however I plan to make a Rust version for fun. "
                "Currently does not support the IRQ feature."
            }
        }
    },
});


static TASTEBASE_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "TasteBase",
    brief:  "A webapp that simplifies the research to make cooking more easy",
    img:    TASTE_IMG,
    desc: {
        html! {
            p {
                "A group project for my software engineering course. I was one of two people in charge with writing and maintaing "
                "the backend. We used MariaDB with custom SpringBoot API for our frontend team to interact with. "
                "Users could log in using a google account, add their ingredints, and get hundreads of recipes to choose from."
                "One fun quirk with this class was that we were tasked with using Jira to follow and develop our projects."
            }
            p { b { "Code Link: "} a href=("https://github.com/TastebaseApp/Tastebase") { "TasteBase" } }
        }
    }
});

static PINGPONG_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "TCP PingPong",
    brief:  "Simple socket programming with both Rust and C",
    img:    PINGPONG_IMG,
    desc: {
        html! {
            p {
                "This project was to get myself introduced to socket programming with both C POSIX and Rust std::net. "
                "It has a Rust server/client and a C server/client communicating back and forth to each other. "
            }
        }
    },
});

static SETUP_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "FreeBSD Setup",
    brief:  "How I use FreeBSD in my day-to-day life",
    img:    SETUP_IMG,
    desc: {
        html! {
            p {
                "I currently have 2 FreeBSD servers I'm actively utilizing. "
                "One of them is being used as a custom router and endpoint. It helps filter and drop unneeded packets (mostly telemetry). "
                "The other is more powerful and runs all my virtual machines, jails, and even my nfs server. "
                "The nfs server is paired with wireguard tunnels allowing me to have a cloud storage for my portable devices. "
            }
        }
    },
});

static WEBSITE_PROJ: LazyLock<Project<'static>> = LazyLock::new(|| Project {
    name:   "Portfolio Website",
    brief:  "This very website you're looking at :)",
    img:    WEBSITE_IMG,
    desc: {
        html! {
            p {
                "This website was built in " b { "pure Rust" } " meaning no HTML, or Javascript used whatsoever! "
                "It was a lot of fun to make, you should check it out :)"
            }
            p { b { "Code Link: "} a href=("https://github.com/Acesp25/portfolio") { "Portfolio" } }
        }
    },
});

struct Project<'a> {
    name: &'a str,
    brief: &'a str,
    img: &'a str,
    desc: Markup,
}
impl<'a> Project<'a> {
    fn showcase(&self) -> Markup {
        let dirty = format!("proj-{}", self.name);
        let toggle_id = dirty
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        let proj_img = format!("/public/images/projects/{}", self.img);

        html! {
            input type="checkbox" id=(toggle_id) class="modal-state";

            label for=(toggle_id) class="project" {
                .info-container {
                    h2 { (self.name)}
                    p { (self.brief) }
                }
                .logo-container {
                    img alt=(self.name) src=(proj_img);
                }
            }

            .modal-overlay {
                label for=(toggle_id) class="modal-backdrop" {};

                .proj-modal-content {
                    label for=(toggle_id) class="close-btn" { "x" }

                    .proj-modal-header {
                        h2 { (self.name) }
                    }
                    .proj-modal-details {
                        p { (self.brief) }
                        (self.desc)
                    }
                }
            }
        }
    }
}
