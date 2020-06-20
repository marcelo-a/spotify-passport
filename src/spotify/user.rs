use std::env;
use serde::Deserialize;
use actix_web::http;
use awc::Client;
use oauth2::basic::BasicClient;
use crate::sp_oauth::auth as auth;
//temp
use actix_web::{web, HttpResponse};

// #[derive(Deserialize)]
pub struct Passport {
    // member variables private by default
    // outside of module user.rs
    // username: String,
    // client_secret: String,
    // client_id: String,
    appclient:Client,
    oauth: BasicClient, //handle oauth and requests
}

impl Passport {
    pub fn new(param: BasicClient) -> Passport {
        Passport {
            appclient: Client::default(),
            oauth: param,
        }
    }

    pub fn oauth(&self) -> &BasicClient {
        &self.oauth
    }

    // pub fn new(usrnm: String, secret: String, id: String, param: BasicClient) -> Passport {
    //     Passport {
    //         username: usrnm,
    //         client_secret: secret,
    //         client_id: id,
    //         client: param,
    //     }
    // }
    // pub fn username(&self) -> &String {
    //     &self.username
    // }

    // pub fn secret(&self) -> &String {
    //     &self.client_secret
    // }

    // pub fn id(&self) -> &String {
    //     &self.client_id
    // }
    pub async fn playlists(&self) /*-> HttpResponse*/ {
        // "GET" "https://api.spotify.com/v1/me/playlists" -H "Accept: application/json" -H "Content-Type: application/json" -H "Authorization: Bearer {}"
        if let Ok(access_token) = env::var("token") {
            let res = &self.appclient
                .get("https://api.spotify.com/v1/me/playlists")
                .header("Authorization", format!("Bearer {}", access_token))
                // "Accept: application/json" -H "Content-Type: application/json"
                .header(http::header::CONTENT_TYPE, "application/json")
                .header(http::header::ACCEPT, "application/json")
                .send() // <- Send http request
                .await; // <- send request and wait for response
            println!("Response: {:?}", res);
        }
        else {
            println!("Cannot retrieve access_token from env!");
        }
        // res
        // let res = &self.appclient
        //     .get("http://www.rust-lang.org") // <- Create request builder
        //     .header("User-Agent", "Actix-web")
        //     .send()                             // <- Send http request
        //     .await;                             // <- send request and wait for response

        // println!("Response: {:?}", res);
    }
}
