use std::env;
use std::collections::{HashSet, HashMap};
use actix:spotify::user::*;

pub fn run(user: User, token: Option<String>, playlist_name: Option<String>) -> Option<HashMap<String, u64>> {
    match token, playlist_name {
        Some(token), Some(playlist_name) => {
            if let Some(artists) : HashSet<String> = spot.getUserArtists(user, token, playlist_name) {
                let mut countries : HashMap<String, u64> = HashMap::new();
                for (name in artists) {
                    let country = musix.searchCountry(name);

                    if countries.contains_key(country) {
                        // *countries.entry(name).or_insert(0) += 1;
                        *countries.get_mut(country).unwrap() += 1;
                    }
                    else {
                        countries.insert(country, 1);
                    }
                }
                // return
                Some(countries)
            }
            else {
                println!("Unable to fetch playlist data! Playlist is either empty or private.");
                None
            } 
        }
        _ => None
    }
}