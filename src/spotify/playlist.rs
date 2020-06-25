use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TracksObject {
    href: String,
    total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageObject {
    height: Option<i64>,
    url: String,
    width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserObject {
    display_name: String,
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    r#type: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistObject {
    collaborative: bool,
    description: String,
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    images: Vec<ImageObject>,
    name: String,
    owner: UserObject,
    primary_color: Option<String>,
    public: Option<bool>,
    snapshot_id: String,
    tracks: TracksObject,
    r#type: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagingObject {
    href: String,
    items: Vec<PlaylistObject>,
    limit: Option<i64>,
    next: Option<String>,
    offset: i64,
    previous: Option<String>,
    total: i64,
}

impl PagingObject {
    pub fn items(&self) -> &Vec<PlaylistObject> {
        &self.items
    }
}

impl PlaylistObject {
    pub fn id(&self) -> &String {
        &self.id
    }
}