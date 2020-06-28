use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct ArtistObject {
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    name: String,
    r#type: String,
    uri: String,
}

impl ArtistObject {
    pub fn external_urls(&self) -> &HashMap<String, String> {
        &self.external_urls
    }

    pub fn href(&self) -> &String {
        &self.href
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn r#type(&self) -> &String {
        &self.r#type
    }

    pub fn uri(&self) -> &String {
        &self.uri
    }
}