macro_rules! relative {
    ($path:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

macro_rules! include_static_unsafe {
    ($path:expr) => {
        include_str!(relative!(concat!("/public", $path)))
    };
}

macro_rules! include_css {
    ($path:expr) => {
        PreEscaped(
            Minifier::default()
                .minify(include_static_unsafe!($path), Level::Three)
                .unwrap(),
        )
    };
}

