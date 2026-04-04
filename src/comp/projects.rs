use maud::{Markup, html};

const RUSTKLD_IMG: &str = "/public/images/projects/rustkld.png";
const VTGPIO_IMG: &str = "/public/images/projects/vtgpio.png";
const SETUP_IMG: &str = "/public/images/projects/setup.jpg";
const PINGPONG_IMG: &str = "/public/images/projects/pingpong.jpg";
const WEBSITE_IMG: &str = "/public/images/projects/website.png";

pub fn projects() -> Markup {
    html! {
        .body-container {
            (project(
                1,
                RUSTKLD_IMG,
                "RustKLD",
                "Rust repo for making FreeBSD device drivers",
                html! {
                    p {
                        "This framework was a result from my GSoC project with FreeBSD. "
                        "It allows developers to easily create their own Rust device drivers for FreeBSD. "
                        "Includes a modular, expandable structure with Cirrus-CI integration and "
                        "documentation to further assist developers wanting to learn and work with it. "
                    }
                    p { b { "Link: "} a href=("https://github.com/Acesp25/rustkld") { "Acesp/rustkld" } }
                }
            ))
            (project(
                2,
                VTGPIO_IMG,
                "VirtIO GPIO",
                "VirtIO General Input/Output Driver for FreeBSD",
                html! {
                    p {
                        "This project was to familarize myself with more technical device drivers with FreeBSD. "
                        "It was designed using the Version 1.3 documentation provided by Oasis-Open. "
                        "Currently does not support the IRQ feature, it is a WIP."
                    } 
                    p { b { "Link: "} a href=("https://github.com/Acesp25/freebsd-src/tree/vtgpio_driver") { "Acesp/vtgpio" } }                  
                }
            ))
            (project(
                3,
                PINGPONG_IMG,
                "PingPong",
                "Simple socket programming with both Rust and C",
                html! {
                    p {
                        "This project was to get myself introducted to socket programming with both C POSIX and Rust std::net. "
                        "It has a Rust server/client and a C server/client communicating back and forth to each other. "
                    }
                    p { b { "Link: "} a href=("https://github.com/Acesp25/pingpong") { "Acesp/pingpong" } }
                }
            ))
            (project(
                4,
                SETUP_IMG,
                "FreeBSD Homelab",
                "My personal FreeBSD setup and how it's used.",
                html! {
                    p {
                        "I currently have two main FreeBSD servers setup. The larger one is used to manage many Bhyve VM's and Jails that are running some silly processes for me. "
                        "The smaller one is used as a NFS, which when combined with my wireguard tunnels allow me to have a personel cloud storage for any of my devices. "
                        "I also have some jails on that one but they're responsible for more lightweight tasks. "
                        "There is a lot more nuance and responsiblites that I give to these servers. I just wanted to show their existence :P "
                    }
                }
            ))
            (project(
                5,
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

fn project(
    id: i32, 
    image: &str, 
    name: &str, 
    brief: &str, 
    details: Markup
) -> Markup {
    let toggle_id = format!("proj-{}", id);
    html! {
        input type="checkbox" id=(toggle_id) class="modal-state";

        label for=(toggle_id) class="project" {
            .info-container {
                h2 { (name)}
                p { (brief) }
            }
            .logo-container {
                img alt=(name) src=(image);
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