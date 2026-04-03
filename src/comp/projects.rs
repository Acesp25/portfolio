use maud::{Markup, html};

const RUSTKLD_IMG: &str = "/public/images/projects/rustkld.png";
const VTGPIO_IMG: &str = "/public/images/projects/vtgpio.png";
const PINGPONG_IMG: &str = "/public/images/projects/pingpong.jpg";
const WEBSITE_IMG: &str = "/public/images/projects/website.png";

pub fn projects() -> Markup {
    html! {
        .body-container {
            (project(
                1,
                RUSTKLD_IMG,
                "RustKLD",
                "Rust framework for making FreeBSD device drivers",
                html! {
                    p {
                        "This framework was a result from my GSoC project with FreeBSD"
                    }
                },
                "https://github.com/Acesp25/rustkld"
            ))
            (project(
                2,
                VTGPIO_IMG,
                "VirtIO GPIO",
                "VirtIO General Input/Output Driver for FreeBSD",
                html! {
                    p {
                        "This project was to familarize myself with more advanced device drivers with FreeBSD. "
                    }                   
                },
                "https://github.com/Acesp25/freebsd-src/tree/vtgpio_driver"
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
                },
                "https://github.com/Acesp25/pingpong"
            ))
            (project(
                4,
                WEBSITE_IMG,
                "Portfolio Website",
                "This very website you're looking at :)",
                html! {
                    p {
                        "This website was build in " b { "pure Rust" } " meaning no HTML, or Javascript used whatsoever!"
                    }
                },
                "https://github.com/Acesp25/portfolio"
            ))
        }
    }
}

fn project(
    id: i32, 
    image: &str, 
    name: &str, 
    brief: &str, 
    details: Markup,
    link: &str
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
                    p { b { "Link: "} a href=(link) { (name) } }
                }
            }
        }
    }
}