use std::env;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use spotify::{user, playlist::PagingObject};

pub fn fetch_info(user: User, token: Option<String>, playlist_name: Option<String>) -> Option<HashMap<String, u64>> {
    let now = Instant::now();

    if let Some(res) = client.playlists().await {
        for playlist in res.items.iter() {
            printl!("{}", playlist.id);
        }
    }
    else {

    }

    match (token, playlist_name) {
        (Some(token), Some(playlist_name)) => {
            if let Some(artists) = spot.getUserArtists(user, token, playlist_name) {
                let mut countries : HashMap<String, u64> = HashMap::new();
                for name in artists.iter() {
                    let country = musix.searchCountry(name);

                    if countries.contains_key(country) {
                        // *countries.entry(name).or_insert(0) += 1;
                        *countries.get_mut(country).unwrap() += 1;
                    }
                    else {
                        countries.insert(country, 1);
                    }
                }
                // benchmark
                let then = (now.elapsed().as_millis() as f64)/100.00;
                println!("{}", then);
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