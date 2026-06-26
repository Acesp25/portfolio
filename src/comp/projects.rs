use maud::{Markup, html};

const RUSTKLD_IMG: &str = "rustkld.png";
const VEB_IMG: &str = "veb.png";
const VTGPIO_IMG: &str = "vtgpio.png";
const SETUP_IMG: &str = "setup.jpg";
const PINGPONG_IMG: &str = "pingpong.jpg";
const WEBSITE_IMG: &str = "website.png";

pub fn projects() -> Markup {
    html! {
        .body-container {
            (project(
                RUSTKLD_IMG,
                "RustKLD",
                "Rust repo for making FreeBSD device drivers",
                html! {
                    p {
                        "This framework was a result from my GSoC project with FreeBSD. "
                        "It allows developers to easily create their own Rust device drivers for FreeBSD. "
                        "Includes a modular, expandable structure with CI integration and "
                        "documentation to further assist developers wanting to learn and work with it. "
                    }
                    p { b { "Link: "} a href=("https://github.com/Acesp25/rustkld") { "Acesp/rustkld" } }
                }
            ))
            (project(
                VEB_IMG,
                "Veb",
                "Virtual Ethernet Bridge network pseudo-device for FreeBSD",
                html! {
                    p {
                        "WIP, I started this project to learn more about network drivers and the network stack altogether."
                    }
                }
            ))
            (project(
                VTGPIO_IMG,
                "VirtIO GPIO",
                "VirtIO General Input/Output Driver for FreeBSD",
                html! {
                    p {
                        "This project was to familiarize myself with more technical device drivers for FreeBSD. "
                        "It was designed using the Version 1.3 documentation provided by Oasis-Open."
                        "I originally wrote it in C, however I plan to make a Rust version for fun."
                        "Currently does not support the IRQ feature, it is a WIP."
                    }
                    p { b { "Link: "} a href=("https://github.com/Acesp25/freebsd-src/tree/vtgpio_driver") { "Acesp/vtgpio" } }
                }
            ))
            (project(
                PINGPONG_IMG,
                "PingPong",
                "Simple socket programming with both Rust and C",
                html! {
                    p {
                        "This project was to get myself introduced to socket programming with both C POSIX and Rust std::net. "
                        "It has a Rust server/client and a C server/client communicating back and forth to each other. "
                    }
                    p { b { "Link: "} a href=("https://github.com/Acesp25/pingpong") { "Acesp/pingpong" } }
                }
            ))
            (project(
                SETUP_IMG,
                "FreeBSD Setup",
                "How I use FreeBSD in my routine",
                html! {
                    p {
                        "I currently have 2 FreeBSD servers I'm activily utilizing. "
                        "One of them is being used as a custom router and endpoint. It helps filter and drop unneeded packets (mostly telemetry). "
                        "The other is more powerful and runs all my virtual machines, jails, and even my nfs server. "
                        "The nfs server is paired with wireguard tunnels allowing me to have a cloud storage for my devices. "
                    }
                }
            ))
            (project(
                WEBSITE_IMG,
                "Portfolio Website",
                "This very website you're looking at :)",
                html! {
                    p {
                        "This website was build in " b { "pure Rust" } " meaning no HTML, or Javascript used whatsoever! "
                        "It was a lot of fun to make, you should check it out :)"
                    }
                    p { b { "Link: "} a href=("https://github.com/Acesp25/portfolio") { "Acesp25/portfolio" } }
                }
            ))
        }
    }
}

fn project(image: &str, name: &str, brief: &str, details: Markup) -> Markup {
    let dirty = format!("proj-{name}");
    let toggle_id = dirty
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    let proj_img = format!("/public/images/projects/{image}");

    html! {
        input type="checkbox" id=(toggle_id) class="modal-state";

        label for=(toggle_id) class="project" {
            .info-container {
                h2 { (name)}
                p { (brief) }
            }
            .logo-container {
                img alt=(name) src=(proj_img);
            }
        }

        .modal-overlay {
            label for=(toggle_id) class="modal-backdrop" {};

            .proj-modal-content {
                label for=(toggle_id) class="close-btn" { "x" }

                .proj-modal-header {
                    h2 { (name) }
                }
                .proj-modal-details {
                    p { (brief) }
                    (details)
                }
            }
        }
    }
}
