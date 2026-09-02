use maud::{Markup, html};

pub enum Tool { 
    C, 
    Rust, 
    Java, 
    Python,
    Assembly,
    Bash,
    SpringBoot, 
    MariaDB, 
    DTrace, 
    Jails, 
    Bhyve, 
    QEMU,
    Pf, 
    WireGuard,
    GDB, 
    Jira,
    Phabricator,
    FreeBSD,
    OpenBSD,
    Linux,
    POSIX,
    NFS,
    Maud,
    Rocket,
}

impl Tool {
    fn label(&self) -> &'static str {
        use Tool::*;
        match &self {
            C => "C",
            Rust => "Rust", 
            Java => "Java", 
            Python => "Python",
            Assembly => "Assembly",
            Bash => "Bash",
            SpringBoot => "SpringBoot", 
            MariaDB => "MariaDB", 
            DTrace => "DTrace", 
            Jails => "Jails", 
            Bhyve => "Bhyve", 
            QEMU => "QEMU",
            Pf => "Pf", 
            WireGuard => "WireGuard",
            GDB => "GDB", 
            Jira => "Jira",
            Phabricator => "Phabricator",
            FreeBSD => "FreeBSD",
            OpenBSD => "OpenBSD",
            Linux => "Linux",
            POSIX => "POSIX",
            NFS => "NFS",
            Maud => "Maud",
            Rocket => "Rocket",
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
