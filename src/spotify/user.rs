use std::collections::HashMap;
use serde::{Serialize, Deserialize};
// local crates
use crate::spotify::playlist::ImageObject;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserObject {
    // country: String, // This field is only available when the current user has granted access to the user-read-private scope.
    display_name: Option<String>, // The name displayed on the user’s profile. null if not available.
    // email: String, // This field is only available when the current user has granted access to the user-read-email scope.
    external_urls: HashMap<String, String>,
    followers: FollowersObject,
    href: String,
    id: String,
    images: Vec<ImageObject>,
    // product: String, // This field is only available when the current user has granted access to the user-read-private scope.
    r#type: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FollowersObject {
    href: Option<String>,
    total: i64,
}