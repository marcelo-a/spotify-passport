/* ---------------- Musixmatch API ---------------- */
#[derive(Default)]
pub struct Musixmatch {
    root: String,
}
use artist::*;

// https://doc.rust-lang.org/std/default/trait.Default.html
impl Default for Musixmatch {
    fn default() -> Self {
        root: String::from("https://api.musixmatch.com/ws/1.1/"),
    }
}

impl Musixmatch {
    fn artist(self) -> &Artist {
        Artist(self)
    }
}