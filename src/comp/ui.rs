use maud::{Markup, html};

pub fn slug(parts: &[&str]) -> String {
    let joined = parts.join("-").to_lowercase();
    let mut out = String::with_capacity(joined.len());
    let mut pending_dash = true;

    for c in joined.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            pending_dash = false;
        } else if !pending_dash {
            out.push('-');
            pending_dash = true;
        }
    }

    out.trim_end_matches('-').to_string()
}

pub struct Modal {
    pub id: String,
    pub card_class: &'static str,
    pub card: Markup,
    pub panel_class: &'static str,
    pub panel: Markup,
}

impl Modal {
    pub fn render(self) -> Markup {
        html! {
            input type="checkbox" id=(self.id) class="modal-state";

            label for=(self.id) class=(self.card_class) { (self.card) }

            .modal-overlay {
                label for=(self.id) class="modal-backdrop" {}

                div class=(self.panel_class) {
                    /* 
                     * tabindex makes the label reachable by keyboard --
                     * labels aren't focusable by default, so without it
                     * there's no way to close this without a mouse.
                     */
                    label for=(self.id) class="close-btn"
                          role="button" tabindex="0" aria-label="Close" { "×" }

                    (self.panel)
                }
            }
        }
    }
}

pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

pub fn link_list(links: &[Link]) -> Markup {
    html! {
        @if !links.is_empty() {
            ul .link-list {
                @for l in links {
                    li { a href=(l.href) { (l.label) } }
                }
            }
        }
    }
}
