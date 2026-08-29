//! Every image on the site goes through this module.
//!
//! One place decides the URL shape, the attribute set, and the wrapper
//! markup, so those can't drift between components. Components supply the
//! filename and the alt text; nothing else.

use maud::{Markup, html};

/// Subdirectory under `/public/images`.
#[derive(Clone, Copy)]
pub enum Dir {
    About,
    Experiences,
    Projects,
    Reading,
    Me,
}

impl Dir {
    const fn as_str(self) -> &'static str {
        match self {
            Dir::About => "about",
            Dir::Experiences => "experiences",
            Dir::Projects => "projects",
            Dir::Reading => "reading",
            Dir::Me => "me",
        }
    }
}

/// Whether the image is above the fold. Anything the visitor has to scroll
/// or switch tabs to reach should be `Lazy`.
#[derive(Clone, Copy)]
pub enum Loading {
    Lazy,
    Eager,
}

impl Loading {
    const fn as_str(self) -> &'static str {
        match self {
            Loading::Lazy => "lazy",
            Loading::Eager => "eager",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Photo {
    dir: Dir,
    file: &'static str,
    alt: &'static str,
}

impl Photo {
    pub const fn about(file: &'static str, alt: &'static str) -> Self {
        Self { dir: Dir::About, file, alt }
    }
    pub const fn experience(file: &'static str, alt: &'static str) -> Self {
        Self { dir: Dir::Experiences, file, alt }
    }
    pub const fn project(file: &'static str, alt: &'static str) -> Self {
        Self { dir: Dir::Projects, file, alt }
    }
    pub const fn book(file: &'static str, alt: &'static str) -> Self {
        Self { dir: Dir::Reading, file, alt }
    }
    pub const fn me(file: &'static str, alt: &'static str) -> Self {
        Self { dir: Dir::Me, file, alt }
    }

    pub fn src(&self) -> String {
        format!("/public/images/{}/{}.webp", self.dir.as_str(), self.file)
    }

    /// A bare `<img>`.
    ///
    /// No width/height attributes: the stylesheet pins an `aspect-ratio`
    /// for every image role, which reserves the box before the file loads
    /// and prevents layout shift on its own. Adding attributes here would
    /// duplicate that, and a wrong pair is worse than none.
    pub fn img(&self, loading: Loading) -> Markup {
        html! {
            img src=(self.src()) alt=(self.alt)
                loading=(loading.as_str()) decoding="async";
        }
    }

    /// For an image that repeats information already in adjacent text.
    /// Hidden from screen readers so it isn't announced twice.
    pub fn decorative(&self, loading: Loading) -> Markup {
        html! {
            img src=(self.src()) alt="" aria-hidden="true"
                loading=(loading.as_str()) decoding="async";
        }
    }

    /// `<img>` inside `.photo-container`. Use this anywhere the image sits
    /// in a flex row -- the wrapper is the flex item that CSS sizes.
    pub fn framed(&self, loading: Loading) -> Markup {
        html! {
            .photo-container { (self.img(loading)) }
        }
    }
}
