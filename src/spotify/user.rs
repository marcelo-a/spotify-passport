// extern crates
use std::{env, time::Instant};
use awc::Client;
use oauth2::basic::BasicClient;
use oauth2::PkceCodeVerifier;
// local crates
use crate::spotify::playlist::PagingObject;

// #[derive(Deserialize)]
pub struct Passport {
    // member variables private by default
    // outside of module user.rs
    // username: String,
    // client_secret: String,
    // client_id: String,
    appclient: Client,
    oauth: BasicClient, //handle oauth and requests
    pkce_verifier: Option<PkceCodeVerifier>,
}

impl Passport {
    pub fn new(param: BasicClient) -> Passport {
        Passport {
            appclient: Client::default(),
            oauth: param,
            pkce_verifier: None,
        }
    }
    
    // pub fn username(&self) -> &String {
    //     &self.username
    // }
    
    // pub fn secret(&self) -> &String {
    //     &self.client_secret
    // }
    
    // pub fn id(&self) -> &String {
    //     &self.client_id
    // }
    
    pub fn oauth(&self) -> &BasicClient {
        &self.oauth
    }

    pub fn set_pkce(&mut self, verifier: PkceCodeVerifier) {
        self.pkce_verifier = Some(verifier);
    }

    pub async fn playlists(&self) -> Option<PagingObject> {
        let now = Instant::now();
        // "GET" "https://api.spotify.com/v1/me/playlists" -H "Accept: application/json" -H "Content-Type: application/json" -H "Authorization: Bearer {}"
        if let Ok(access_token) = env::var("token") {
            match self.appclient
                .get("https://api.spotify.com/v1/me/playlists")
                .bearer_auth(access_token)
                .content_type("application/json")
                .header("Accept", "application/json")
                .send().await { // <- send http request and wait for response
                    // check if successful
                    Ok(mut res) => {
                        if let Ok(json) = res.json::<PagingObject>().await {
                            assert!(res.status().is_success());
                            println!("Response retrieved");
                            return Some(json)
                        }
                        else {
                            panic!("JSON parsing error!");
                        }
                    },
                    // response error!
                    Err(e) => {
                        panic!("GET request error! {}", e);
                    }
                }
        }
        else {
            panic!("Cannot retrieve access_token from env!");
        }

        let then : f64 = (now.elapsed().as_millis() as f64) / 1000.0;
        println!("{}", then);

        return None
    }

    /*pub async fn tracks(&self, playlist_id: String) /*-> HttpResponse*/ {
        // GET https://api.spotify.com/v1/playlists/{playlist_id}/tracks
        if let Ok(access_token) = env::var("token") {
            let response = self.appclient
                .get(format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
                // -H "Authorization: Bearer {}"
                .bearer_auth(access_token)
                // "Accept: application/json" -H "Content-Type: application/json"
                .content_type("application/json")
                .header("Accept", "application/json")
                .send() // <- Send http request
                .await; // <- send request and wait for response
                //     let mut x = ResBody {
                //         body: response.json(),
                //         // body: String::from(response.json()),
                //     };
                //     println!("Response: {:#?}", x);
                // } 
            
            // HttpResponse::Ok()
            //     .content_type("application/json")
            //     .body("{}")
            // println!("Response: {:#?}", response.unwrap().body().into());
            // response.unwrap().body().and_then(|bytes| {
            //     println!("{:?}", bytes);
            //     Ok(())
            // });
        }
        else {
            println!("Cannot retrieve access_token from env!");
        }
    }*/
}
