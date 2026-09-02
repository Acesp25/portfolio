use maud::{Markup, html};
use std::sync::LazyLock;

use super::images::{Loading, Photo};
use super::ui::{Link, Modal, link_list, slug};

pub fn projects() -> Markup {
    html! {
        .body-container {
            (VEB.showcase())
            (RUSTKLD.showcase())
            (VTGPIO.showcase())
            (TASTEBASE.showcase())
            (PINGPONG.showcase())
            (SETUP.showcase())
            (WEBSITE.showcase())
        }
    }
}

static RUSTKLD: LazyLock<Project> = LazyLock::new(|| Project {
    name: "RustKLD",
    brief: "FreeBSD framework for writing Rust kernel device drivers",
    img: Photo::project("rustkld", "RustKLD project banner"),
    desc: html! {
        p {
            "This framework came out of my GSoC project with FreeBSD. It lets developers "
            "create their own Rust device drivers for FreeBSD, with a modular, expandable "
            "structure, CI integration, and documentation for people learning the codebase."
        }
    },
    links: vec![Link {
        label: "Code: Acesp25/rustkld",
        href: "https://github.com/Acesp25/rustkld",
    }],
});

static VEB: LazyLock<Project> = LazyLock::new(|| Project {
    name: "veb(4) Network Driver (WIP)",
    brief: "Virtual Ethernet Bridge pseudo-device for FreeBSD",
    img: Photo::project("veb", "veb(4) project banner"),
    desc: html! {
        p {
            "A FreeBSD network driver modelled on OpenBSD's veb(4). It is structurally "
            "similar to if_bridge(4) but stripped down for jails and bhyve VMs, and it "
            "hides itself from the host network stack. The result is a shorter L2 path, "
            "which matters for latency-sensitive workloads."
        }
    },
    links: vec![],
});

static VTGPIO: LazyLock<Project> = LazyLock::new(|| Project {
    name: "VirtIO GPIO Driver (WIP)",
    brief: "VirtIO general-purpose I/O driver for FreeBSD",
    img: Photo::project("vtgpio", "VirtIO GPIO project banner"),
    desc: html! {
        p {
            "Written to get familiar with more involved FreeBSD device drivers, against "
            "the OASIS VirtIO 1.3 specification. Originally in C; a Rust version is "
            "planned. Does not yet support the IRQ feature."
        }
    },
    links: vec![],
});

static TASTEBASE: LazyLock<Project> = LazyLock::new(|| Project {
    name: "TasteBase",
    brief: "A web app that cuts the research out of cooking",
    img: Photo::project("tastebase", "TasteBase project banner"),
    desc: html! {
        p {
            "A group project for my software engineering course. I was one of two people "
            "writing and maintaining the backend: MariaDB behind a custom Spring Boot API "
            "for the frontend team to consume. Users signed in with a Google account, "
            "entered their ingredients, and got matching recipes. The course also had us "
            "running the whole project through Jira."
        }
    },
    links: vec![Link {
        label: "Code: TastebaseApp/Tastebase",
        href: "https://github.com/TastebaseApp/Tastebase",
    }],
});

static PINGPONG: LazyLock<Project> = LazyLock::new(|| Project {
    name: "TCP PingPong",
    brief: "Socket programming in both Rust and C",
    img: Photo::project("pingpong", "TCP PingPong project banner"),
    desc: html! {
        p {
            "An introduction to socket programming with C POSIX sockets and Rust's "
            "std::net. A Rust server/client and a C server/client talk to each other in "
            "both directions."
        }
    },
    links: vec![],
});

static SETUP: LazyLock<Project> = LazyLock::new(|| Project {
    name: "FreeBSD Setup",
    brief: "How I run FreeBSD day to day",
    img: Photo::project("setup", "Photo of my FreeBSD home servers"),
    desc: html! {
        p {
            "Two FreeBSD servers in active use. One is a custom router and endpoint that "
            "filters and drops unwanted traffic, mostly telemetry. The other is heavier "
            "and runs my virtual machines, jails, and an NFS server. The NFS server is "
            "paired with WireGuard tunnels, which gives me cloud storage for my portable "
            "devices."
        }
    },
    links: vec![],
});

static WEBSITE: LazyLock<Project> = LazyLock::new(|| Project {
    name: "Portfolio Website",
    brief: "This very website you're looking at :)",
    img: Photo::project("website", "Screenshot of this website"),
    desc: html! {
        p {
            "Built in Rust with no hand-written HTML and no JavaScript, the templates "
            "are Rust macros (Maud) served by Rocket. It was a lot of fun to make, you "
            "should check it out :)"
        }
    },
    links: vec![Link {
        label: "Code: Acesp25/portfolio",
        href: "https://github.com/Acesp25/portfolio",
    }],
});

struct Project {
    name:   &'static str,
    brief:  &'static str,
    img:    Photo,
    desc:   Markup,
    links:  Vec<Link>,
}

impl Project {
    fn showcase(&self) -> Markup {
        Modal {
            id: slug(&["proj", self.name]),
            card_class: "project",
            card: html! {
                .info-container {
                    h2 { (self.name) }
                    p { (self.brief) }
                }
                (self.img.framed(Loading::Lazy))
            },
            panel_class: "proj-modal-content",
            panel: html! {
                .proj-modal-header {
                    h2 { (self.name) }
                }
                .proj-modal-details {
                    p { (self.brief) }
                    (self.desc)
                    (link_list(&self.links))
                }
            },
        }
        .render()
    }
}
