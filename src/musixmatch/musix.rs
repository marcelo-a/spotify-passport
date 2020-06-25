use std::env;
use artist::{Artist};

/* ---------------- Musixmatch API ---------------- */
#[derive(Default)]
pub struct Musixmatch {
    root: String,
    api_key, String,
}

// https://doc.rust-lang.org/std/default/trait.Default.html
impl Default for Musixmatch {
    fn default() -> Self {
        root: String::from("https://api.musixmatch.com/ws/1.1/"),
        api_key: env::var("MUSIXMATCH_API_KEY"),
    }
}

impl Musixmatch {
    fn artist(self) -> &Artist {
        Artist(self)
    }
}