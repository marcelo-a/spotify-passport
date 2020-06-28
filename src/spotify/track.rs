use std::collections::HashMap;
use serde::{Serialize, Deserialize};
// local crates
// use crate::spotify::playlist::ImageObject;
use crate::spotify::artist::ArtistObject;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Track {
    // album: AlbumObject,
    pub artists: Vec<ArtistObject>,
    // available_markets: Vec<String>,
    // disc_number: i64,
    // duration_ms: i64,
    // explicit: bool,
    // external_ids: HashMap<String, String>,
    // external_urls: HashMap<String, String>,
    // href: String,
    // id: String,
    // is_playable: bool,
    // linked_from: TrackLinkObject,
    // restrictions: HashMap<String, String>,
    // name: String,
    // popularity: i64,
    // preview_url: String,
    // track_number: i64,
    // r#type: String,
    // uri: String,
    // is_local: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackLinkObject {
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    r#type: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct Tracks {
    pub track: Track,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct TrackPagingObject {
    pub items: Vec<Tracks>,
    next: Option<String>,
}

impl TrackPagingObject {
    pub fn next(&self) -> &Option<String> {
        &self.next
    }
}