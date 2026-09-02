use maud::{Markup, html};
use std::sync::LazyLock;

use super::images::{Loading, Photo};
use super::ui::{Link, Modal, link_list, slug};
use super::tools::{Tool, Tool::*};

pub fn projects() -> Markup {
    html! {
        .body-container {
            (VEB.showcase())
            (RUSTKLD.showcase())
            (VTGPIO.showcase())
            (TASTEBASE.showcase())
            (SETUP.showcase())
            (WEBSITE.showcase())
            (PINGPONG.showcase())
        }
    }
}


static VEB: LazyLock<Project> = LazyLock::new(|| Project {
    name: "veb(4) Network Driver (WIP)",
    brief: "Virtual Ethernet Bridge pseudo-device for FreeBSD",
    img: Photo::project("veb", "veb(4) project banner"),
    tools: vec![C, DTrace, Bhyve, Jails, FreeBSD],
    desc: html! {
        p {
            "A Layer 2 bridge for connecting jails and bhyve VMs, ported from OpenBSD. "
            "It is structurally close to FreeBSD's own if_bridge(4), but stripped down: "
            "it does no protocol processing and never attaches to the host network stack."
        }
        ul {
            li {
                "Shorter packet path than if_bridge(4), which matters for "
                "latency-sensitive workloads."
            }
            li { 
                "Incorperates extrinsic L2 Host communication via a vport interface, "
                "allowing a more efficient bridging technique for members interfaces." }
            li { "Completly isolated from the host network stack." }
            li {
                "Currently in development, heading for design review on the "
                "freebsd-net mailing list."
            }
        }
    },
    links: vec![Link {
        label: "Code: Acesp/freebsd-veb",
        href: "https://github.com/Acesp25/freebsd-veb"
    }],
});

static RUSTKLD: LazyLock<Project> = LazyLock::new(|| Project {
    name: "RustKLD",
    brief: "FreeBSD framework for writing Rust kernel device drivers",
    img: Photo::project("rustkld", "RustKLD project banner"),
    tools: vec![Rust, C, FreeBSD, Bhyve, Jails],
    desc: html! {
        p {
            "My Google Summer of Code 2025 project with FreeBSD. It gives a developer "
            "the scaffolding to write a kernel driver in Rust without first solving the "
            "build and interop problems on their own."
        }
        ul {
            li {
                "Modular layout, so a new driver starts from a working template rather "
                "than an empty file."
            }
            li {
                "Includes custom wrappers and C object handling, taking full advantage of the safe and powerful design "
                "of Rust's type system."
            }
            li {
                "Handles the module build glue and the boundary between Rust and the "
                "C kernel APIs."
            }
            li { "CI integration and documentation written for people new to the codebase." }
            li { "Spearheaded project architecture and development under two FreeBSD committer mentors." }
        }
    },
    links: vec![Link {
        label: "Code: Acesp25/rustkld",
        href: "https://github.com/Acesp25/rustkld",
    }],
});

static VTGPIO: LazyLock<Project> = LazyLock::new(|| Project {
    name: "VirtIO GPIO Driver (WIP)",
    brief: "VirtIO general-purpose I/O driver for FreeBSD",
    img: Photo::project("vtgpio", "VirtIO GPIO project banner"),
    tools: vec![C, DTrace, Bhyve, QEMU, FreeBSD],
    desc: html! {
        p {
            "A driver for the VirtIO GPIO device, written against the OASIS VirtIO 1.3 "
            "specification to get hands-on with a more involved driver than a toy example."
        }
        ul {
            li { "Implements the specification's core GPIO operations over the VirtIO transport." }
            li { "IRQ support is not implemented yet, and a Rust version is planned." }
        }
    },
    links: vec![],
});

static TASTEBASE: LazyLock<Project> = LazyLock::new(|| Project {
    name: "TasteBase",
    brief: "A web app that cuts the research out of cooking",
    img: Photo::project("tastebase", "TasteBase project banner"),
    tools: vec![Java, SpringBoot, Jira, MariaDB],
    desc: html! {
        p {
            "A recipe app built by a team for my software engineering course. You enter "
            "the ingredients you already have and it returns recipes you can actually "
            "cook. I was one of two people writing and maintaining the backend."
        }
        ul {
            li {
                "Spring Boot REST API in a layered controller, service, and DAO "
                "structure, backed by MariaDB."
            }
            li { "Google sign-in through Spring Security OAuth2, with JWTs for session handling." }
            li { "JUnit 5 tests against an in-memory H2 database, endpoints documented with OpenAPI." }
            li { "Run as a sprint-based project in Jira as part of the coursework." }
        }
    },
    links: vec![Link {
        label: "Code: TastebaseApp/Tastebase",
        href: "https://github.com/TastebaseApp/Tastebase",
    }],
});

static SETUP: LazyLock<Project> = LazyLock::new(|| Project {
    name: "FreeBSD Setup",
    brief: "How I run FreeBSD day to day",
    img: Photo::project("setup", "Photo of my FreeBSD home servers"),
    tools: vec![FreeBSD, Pf, WireGuard, Jails, Bhyve, NFS],
    desc: html! {
        p {
            "Two FreeBSD servers I run at home, and where most of my hands-on systems "
            "experience comes from."
        }
        ul {
            li {
                "A router and network endpoint running pf and AdGuardHome, filtering and dropping "
                "unwanted outbound traffic, mostly telemetry."
            }
            li { "A second, heavier host running my bhyve virtual machines and jails." }
            li { "Bhyve VMs are tailored for compiling and testing my written FreeBSD code."}
            li { "The jails that are setup include by are not limited to: hosting this website, "
                 "hosting image viewers for my NFS server, and hosting a modded Minecraft server for my friends and I."
            }
            li {
                "NFS exported over WireGuard tunnels, which gives my portable devices "
                "storage that follows them off the network."
            }
            li { "Daily ZFS snapshots via to ensure that data sent to and from my NFS can get restored upon a failure."}
        }
    },
    links: vec![],
});

static WEBSITE: LazyLock<Project> = LazyLock::new(|| Project {
    name: "Portfolio Website",
    brief: "This very website you're looking at :)",
    img: Photo::project("website", "Screenshot of this website"),
    tools: vec![Rust, Maud, Rocket, FreeBSD],
    desc: html! {
        p {
            "This site, written in Rust with no hand-written HTML and no JavaScript, it "
            "was a lot of fun to make!"
        }
        ul {
            li {
                "Templates are Rust macros (Maud), type-checked and compiled into the "
                "binary alongside everything else, and served by Rocket."
            }
            li { "CSS is minified and images are converted to WebP ahead of deployment." }
            li { "Runs on my own FreeBSD hardware." }
        }
    },
    links: vec![Link {
        label: "Code: Acesp25/portfolio",
        href: "https://github.com/Acesp25/portfolio",
    }],
});

static PINGPONG: LazyLock<Project> = LazyLock::new(|| Project {
    name: "TCP PingPong",
    brief: "Socket programming in both Rust and C",
    img: Photo::project("pingpong", "TCP PingPong project banner"),
    tools: vec![C, POSIX, FreeBSD],
    desc: html! {
        p {
            "A small exercise to learn socket programming from both sides: C POSIX "
            "sockets and Rust's std::net. A Rust server and client and a C server and "
            "client all talk to each other in either direction."
        }
    },
    links: vec![],
});

struct Project {
    name:   &'static str,
    brief:  &'static str,
    img:    Photo,
    tools:  Vec<Tool>,
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
                    (Tool::link_tools(&self.tools))
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
                    (Tool::link_tools(&self.tools))
                    (link_list(&self.links))
                }
            },
        }
        .render()
    }
}
