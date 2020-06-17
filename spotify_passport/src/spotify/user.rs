use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    // member variables private by default
    // outside of module user.rs
    username: String,
    client_secret: String,
    client_id: String,
}

impl User {
    fn username(&self) -> &String {
        &self.username
    }

    fn secret(&self) -> &String {
        &self.client_secret
    }

    fn id(&self) -> &String {
        &self.client_id
    }
}