use maud::{Markup, html};

macro_rules! tools {
    ($($variant:ident $(= $label:literal)?),* $(,)?) => {
        pub enum Tool { $($variant),* }

        impl Tool {
            fn label(&self) -> &'static str {
                match self {
                    $(Tool::$variant => tools!(@label $variant $(, $label)?)),*
                }
            }

            pub fn link_tools(tools: &[Tool]) -> Markup {
                html! {
                    @if !tools.is_empty() {
                        .tools-list {
                            p {
                                strong {"Tools: "}
                                @for (i, t) in tools.iter().enumerate() {
                                    @if i > 0 { ", " }
                                        (t.label())
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    (@label $variant:ident) => { stringify!($variant) };
    (@label $variant:ident, $label:literal) => { $label };
}

tools! {
    C, Rust, Java, Python, Assembly, Bash,
    SpringBoot = "Spring Boot", MariaDB, PyTorch, TensorFlow, Maud, Rocket, POSIX,
    DTrace, Jails, Bhyve = "bhyve", QEMU, Pf = "pf", GDB = "gdb",
    Jira, Phabricator, 
    FreeBSD, OpenBSD, Linux, 
    NFS, WireGuard,
}
