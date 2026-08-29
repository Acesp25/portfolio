use maud::{Markup, html};

use super::images::{Loading, Photo};

pub const AURA: Photo = Photo::about("aura", "Me aura farming");
pub const FENCING: Photo = Photo::about("fencing", "Me in my foil fencing gear");
pub const AE_APEX: Photo = Photo::about("ae-apex", "An After Effects composition mid-edit");

pub fn about() -> Markup {
    html! {
        .body-container {
            (stuff_photo(&AURA, html! {
                h3 { "About Me" }
                p {
                    "I am a student at Texas State University pursuing Computer Science, "
                    "Applied Math, and Chinese. I'm currently a deep learning researcher "
                    "for my university while making FreeBSD-based projects on the side."
                }
                h3 { "Resumes" }
                ul .resume-list {
                    li {
                        a href="/public/docs/resumes/ace-main.pdf"
                          download="aaron-espinoza-resume.pdf" {
                            "Download General Resume"
                        }
                    }
                    li {
                        a href="/public/docs/resumes/ace-sys.pdf"
                          download="aaron-espinoza-resume-systems.pdf" {
                            "Download Systems Engineering Resume"
                        }
                    }
                    li {
                        a href="/public/docs/resumes/ace-ai.pdf"
                          download="aaron-espinoza-resume-ai.pdf" {
                            "Download AI Research Resume"
                        }
                    }
                }
            }))

            (stuff(html! {
                h3 { "Technical Stack" }
                p { strong { "Languages: " }
                    "C, Rust, Python, Java, Bash, Assembly (MIPS, x86-64)" }
                p { strong { "Systems Programming: " }
                    "FreeBSD drivers, POSIX (pthreads, sockets, IPC), ATF and Kyua testing" }
                p { strong { "AI & ML: " }
                    "PyTorch, TensorFlow, UNets, GANs" }
                p { strong { "Networking: " }
                    "pf, iptables, Unbound, Tailscale" }
                p { strong { "Environments & Virtualization: " }
                    "FreeBSD, Linux, bhyve, QEMU, jails" }
                p { strong { "Tools: " }
                    "Vim, Visual Studio Code, Git, DTrace, GDB, Wireshark" }
            }))

            (photo_stuff(&FENCING, html! {
                h3 { "Away from my text editor" }
                p {
                    "When I'm not looking at code, I enjoy competing in a wide range of "
                    "sports. My current fixations are foil fencing, soccer, tennis, and "
                    "swimming. I also enjoy reading books about my favorite computer "
                    "topics, and playing competitive video games."
                }
            }))

            (stuff_photo(&AE_APEX, html! {
                h3 { "More fun things" }
                p {
                    "Having a creative outlet is very important to me. When it's not "
                    "programming, I love expressing my creativity by editing videos in "
                    "After Effects, though most of the projects I start end up scrapped."
                }
            }))
        }
    }
}

fn stuff(body: Markup) -> Markup {
    html! {
        .stuff { (body) }
    }
}

fn stuff_photo(photo: &Photo, body: Markup) -> Markup {
    html! {
        .stuff-photo {
            .info-container { (body) }
            (photo.framed(Loading::Lazy))
        }
    }
}

fn photo_stuff(photo: &Photo, body: Markup) -> Markup {
    html! {
        .photo-stuff {
            (photo.framed(Loading::Lazy))
            .info-container { (body) }
        }
    }
}
