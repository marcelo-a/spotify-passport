// warnings
#![allow(unreachable_code, unused_imports, unused_mut)]
// extern crates
use std::{env, collections::{HashSet, HashMap}};
use awc::Client;
use oauth2::{basic::BasicClient, PkceCodeVerifier};
// local crates
use crate::spotify::playlist::{PagingObject, PlaylistObject};
use crate::spotify::user::{UserObject};
use crate::spotify::track::{TrackPagingObject};

pub struct Passport {
    appclient: Client,
    oauth: BasicClient, //handle oauth and requests
    pkce_verifier: Option<PkceCodeVerifier>,
}

impl Passport {
    pub fn new(param: BasicClient) -> Passport {
        Passport {
            appclient: Client::default(),
            oauth: param,
            // user_playlists: None,
            pkce_verifier: None,
        }
    }
    
    pub fn oauth(&self) -> &BasicClient {
        &self.oauth
    }

    pub fn set_pkce(&mut self, verifier: PkceCodeVerifier) {
        self.pkce_verifier = Some(verifier);
    }

    pub async fn current_user_profile(&self) -> Option<UserObject> {
        // GET "https://api.spotify.com/v1/me" -H "Authorization: Bearer {your access token}"
        if let Ok(access_token) = env::var("token") {
            match self.appclient
                .get("https://api.spotify.com/v1/me")
                .bearer_auth(access_token)
                .content_type("application/json")
                .header("Accept", "application/json")
                .send().await { // <- send http request and wait for response
                    // check if successful
                    Ok(mut res) => {
                        if let Ok(json) = res.json::<UserObject>().await {
                            assert!(res.status().is_success());
                            return Some(json)
                        }
                        else {
                            println!("JSON parsing error!");
                            return None
                        }
                    },
                    // response error!
                    Err(e) => {
                        println!("GET Spotify current user profile request error! {}", e);
                    }
                }
        }
        else {
            println!("Cannot retrieve access_token from env!");
        }

        return None
    }

    pub async fn retrieve_all_playlists(&self) -> Option<HashMap<String, String>> {
        if let Some(mut res) = self.get_playlists().await {
            // (key, value) = id, name
            let mut map : HashMap<String, String> = HashMap::new();
            for playlist in res.items().iter() {
                map.insert(playlist.id().to_string(), playlist.name().to_string());
            }

            while let Some(next_set) = self.next(res.next()).await {
                for playlist in next_set.items().iter() {
                    map.insert(playlist.id().to_string(), playlist.name().to_string());
                }
                res = next_set;
            }

            return Some(map);
        }
        return None
    }

    pub async fn get_playlists(&self) -> Option<PagingObject> {
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
                            // println!("Response retrieved");
                            return Some(json)
                        }
                        else {
                            println!("JSON parsing error!");
                            return None
                        }
                    },
                    // response error!
                    Err(e) => {
                        println!("GET Spotify playlists request error! {}", e);
                    }
                }
        }
        else {
            println!("Cannot retrieve access_token from env!");
        }

        return None
    }

    pub async fn next(&self, next_list: &Option<String>) -> Option<PagingObject> {
        if let Ok(access_token) = env::var("token") {
            if let Some(next_set) = next_list {
                match self.appclient
                    .get(next_set)
                    .bearer_auth(access_token)
                    .content_type("application/json")
                    .header("Accept", "application/json")
                    .send().await { // <- send http request and wait for response
                        Ok(mut res) => {
                            if let Ok(json) = res.json::<PagingObject>().await {
                                // check for success
                                assert!(res.status().is_success());
                                return Some(json)
                            }
                            else {
                                println!("JSON parsing error!");
                                return None
                            }
                        },
                        // response error!
                        Err(e) => {
                            println!("Spotify GET next request error! {}", e);
                            return None
                        }
                    }
            }
            else {
                // println!("Next is null");
                return None
            }
        }
        else {
            println!("Cannot retrieve access_token from env!");
            return None
        }
    }

    pub async fn playlist_artists(&self, playlist_id: String) -> Option<TrackPagingObject> {
        // GET https://api.spotify.com/v1/playlists/{playlist_id}/tracks
        if let Ok(access_token) = env::var("token") {
            match self.appclient
                .get(format!("https://api.spotify.com/v1/playlists/{}/tracks", playlist_id))
                .bearer_auth(access_token)
                .query(&[("fields", "next, items(track(artists(name)))")]).unwrap() // fields=items(track(artists))
                .content_type("application/json")
                .header("Accept", "application/json")
                .send().await { // <- send http request and wait for response
                    Ok(mut res) => {
                        if let Ok(json) = res.json::<TrackPagingObject>().await {
                            assert!(res.status().is_success());
                            return Some(json)
                        }
                        else {
                            println!("JSON parsing error!");
                            return None
                        }
                    },
                    // response error!
                    Err(e) => {
                        println!("Spotify GET playlist artists request error! {}", e);
                        return None
                    }
                }
        }
        else {
            println!("Cannot retrieve access_token from env!");
        }

        return None
    }

    pub async fn next_artists(&self, next_set: &Option<String>) -> Option<TrackPagingObject> {
        if let Ok(access_token) = env::var("token") {
            if let Some(next_url) = next_set {
                match self.appclient
                    .get(next_url)
                    .bearer_auth(access_token)
                    .content_type("application/json")
                    .header("Accept", "application/json")
                    .send().await { // <- send http request and wait for response
                        Ok(mut res) => {
                            if let Ok(json) = res.json::<TrackPagingObject>().await {
                                // check for success
                                assert!(res.status().is_success());
                                return Some(json)
                            }
                            else {
                                println!("JSON parsing error!");
                                return None
                            }
                        },
                        // response error!
                        Err(e) => {
                            println!("Spotify GET next artist page request error! {}", e);
                            return None
                        }
                    }
            }
            else {
                // println!("Next is null");
                return None
            }
        }
        else {
            println!("Cannot retrieve access_token from env!");
            return None
        }
    }

    pub async fn fetch_artists(&self, desired_id: &String) -> Option<HashMap<String, u64>> {
        let mut map : HashMap<String, u64> = HashMap::new();
        if let Some(mut temp) = self.playlist_artists(desired_id.to_string()).await {
            for obj in temp.items.iter() {
                for artist in obj.track.artists.iter() {
                    if !map.contains_key(artist.name()) {
                        map.insert(artist.name().to_string(), 1);
                    }
                    else {
                        *map.get_mut(artist.name()).unwrap() += 1;
                    }
                }
            }

            while let Some(next) = self.next_artists(temp.next()).await {
                for obj in next.items.iter() {
                    for artist in obj.track.artists.iter() {
                        if !map.contains_key(artist.name()) {
                            map.insert(artist.name().to_string(), 1);
                        }
                        else {
                            *map.get_mut(artist.name()).unwrap() += 1;
                        }
                    }
                }
                temp = next;
            }
            // println!("{:?}", a);
            // let json_struct = serde_json::to_string(&a).unwrap();
            // return Some(json_struct);
            return Some(map);
        }
        return None;
    }
}
