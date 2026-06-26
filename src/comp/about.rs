use maud::{Markup, html};

const AURA_IMG:     &str = "aura.jpeg";
const FENCING_IMG:  &str = "fencing.jpg";
const AEAPEX_IMG:   &str = "ae-apex.png";

pub fn about() -> Markup {
    html! {
        .body-container {
           (stuff_photo(
                AURA_IMG,
                html! {
                    h3 { "About Me" }
                    p {
                        "I am a student at Texas State University pursuing Computer Science, "
                        "Applied Math, and Chinese. I currently am a deep learning researcher for my university "
                        "while making FreeBSD-based projects on the side. "
                    }
                    h3 { "Resumes "}
                    a href="/public/docs/resumes/ace-main.pdf" download="aaron-espinoza-resume.pdf" {
                        p {"Download General Resume"}
                    }
                    p {} 
                    a href="/public/docs/resumes/ace-sys.pdf" download="aaron-espinoza-resume.pdf" {
                        p {"Download Systems Engineering Resume"}
                    }
                    p {} 
                    a href="/public/docs/resumes/ace-ai.pdf" download="aaron-espinoza-resume.pdf" {
                        p {"Download AI Research Resume"}
                    }
                }
           ))
           (stuff(
                html! {
                    h3 { "Technical Stack" }
                    p {
                        b { "Languages: " }
                        "C, Rust, Python, Java, Bash, Assembly (Mips, x86/64) "
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
                        "Vim, Visual Studio Code, Git, DTrace, GDB, Wireshark "
                    }
                }
           ))
           (photo_stuff(
                FENCING_IMG,
                html! {
                    h3 { "Away from my text editor" }
                    p {
                        "When I'm not looking at code, I enjoy competing in a vast majority of sports. "
                        "My current fixations are fencing foil, soccer, tennis, and swimming. "
                        "I also enjoy reading books about my favorite computer topics, and playing "
                        "competitive video games. "
                    }
                }
           ))
           (stuff_photo(
               AEAPEX_IMG,
               html! {
                   h3 { "More fun things" }
                   p {
                       "Having a creative outlet is very important to me. Whenever it's not programming, "
                       "I love expressing my creativity through editing videos in After Effects. "
                       "Although, most of the projects I work on end up being scrapped. "
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
    let pic = format!("/public/images/about/{photo}");

    html! {
        .stuff-photo {
            .info-container {
                (stuff)
            }
            .logo-container {
                img src=(pic) alt="Photo";
            }
        }
    }
}

fn photo_stuff(photo: &str, stuff: Markup) -> Markup {
    let pic = format!("/public/images/about/{photo}");

    html! {
        .photo-stuff {
            .logo-container {
                img src=(pic) alt="Image";
            }
            .info-container {
                (stuff)
            }
        }
    }
}
