use maud::{Markup, html};

const FACE_IMG: &str = "/public/images/about/face.jpg";
const AURA_IMG: &str = "/public/images/about/aura.jpeg";
const FENCING_IMG: &str = "/public/images/about/fencing.jpg";

pub fn about() -> Markup {
    html! {
        .body-container {
           (stuff_photo(
                FACE_IMG,
                html! {
                    h3 { "About Me"}
                    p {
                        "Not sure what to put here for now ngl, I do things, I like things, I am a person. "
                    }
                }
           ))
           (photo_stuff(
                FENCING_IMG,
                html! {
                    h3 { "Away from the computer" }
                    p {
                        "When im not looking a computer screen, I enjoy competing in a vast majority of sports. "
                        "Especially fencing, soccer, tennis, and swimming. "
                        "I also enjoy reading books about my favorite computer topics, and occationally playing competitive video games. "
                    }
                }
           ))
           (stuff_photo(
                AURA_IMG,
                html! {
                    h3 { "" }
                }
           ))
           (stuff(
                html! {
                    h3 { "Technical Stack" }
                    p { 
                        b { "Languages: " } 
                        "Rust, C, Python, Java, Bash, Assembly (x86/64) " 
                    }
                    p { 
                        b { "Systems Programming: " } 
                        "FreeBSD Drivers, POSIX (pthreads, sockets, IPC), ATF and Kyua testing " 
                    }
                    p { 
                        b { "AI & ML: " } 
                        "PyTorch, TensorFlow, UNets, GANs " 
                    }
                    p {
                        b { "Networking: " }
                        "PF, iptables, Unbound, Tailscale "
                    }
                    p { 
                        b { "Environments & Virtualization: " } 
                        "FreeBSD, Linux, Bhyve, QEMU, Jails " 
                    }
                    p { 
                        b { "Tools: " } 
                        "Vim, Visual Studio Code, Git, DTrace, Wireshark " 
                    }
                }
           ))
        }
    }
}

fn stuff(stuff: Markup) -> Markup {
    html! {
        .stuff {
            (stuff)
        }
    }
}

fn stuff_photo(photo: &str, stuff: Markup) -> Markup {
    html! {
        .stuff-photo {
            .info-container {
                (stuff)
            }
            .logo-container {
                img src=(photo) alt="Photo";
            }            
        }
    }
}

fn photo_stuff(photo: &str, stuff: Markup) -> Markup {
    html! {
        .photo-stuff {
            .logo-container {
                img src=(photo) alt="Image";
            }
            .info-container {
                (stuff)
            }
        }
    }
}