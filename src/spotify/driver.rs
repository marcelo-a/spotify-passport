use std::env;
use std::collections::{HashSet, HashMap};
use actix_web::web;
use actix_session::Session;
// local crates
use crate::spotify::*;

// temp
use std::time::Instant;

// pub fn fetch_info(session: Session, user: User, playlist_name: Option<String>) -> Option<HashMap<String, u64>> {
//     match (token, playlist_name) {
//         (Some(token), Some(playlist_name)) => {
//             if let Some(artists) = spot.getUserArtists(user, token, playlist_name) {
//                 let mut countries : HashMap<String, u64> = HashMap::new();
//                 for name in artists.iter() {
//                     let country = musix.searchCountry(name);

//                     if countries.contains_key(country) {
//                         // *countries.entry(name).or_insert(0) += 1;
//                         *countries.get_mut(country).unwrap() += 1;
//                     }
//                     else {
//                         countries.insert(country, 1);
//                     }
//                 }
//                 // benchmark
//                 let then = (now.elapsed().as_millis() as f64)/100.00;
//                 println!("{}", then);
//                 // return
//                 Some(countries)
//             }
//             else {
//                 println!("Unable to fetch playlist data! Playlist is either empty or private.");
//                 None
//             } 
//         }
//         _ => None
//     }
// }

pub async fn get_artists(session: Session, spotify: web::Data<api::Passport>) {
    if let Some(token) = session.get::<String>("token").unwrap() {
        let now = Instant::now();

        if let Some(mut res) = spotify.playlists().await {

            for playlist in res.items().iter() {
                println!("{}", playlist.id());
                if let Some(mut temp) = spotify.playlist_artists(playlist.id().to_string()).await {
                    for obj in temp.items.iter() {
                        for artist in obj.track.artists.iter() {
                            println!("{}", artist.name());
                        }
                    }
                }
            }

            while let Some(page) = spotify.next(&res).await {
                for play in page.items().iter() {
                    println!("{}", play.id());
                }
                res = page;
            }
        }
    }
}

// if let Some(token) = session.get::<String>("token").unwrap() {

//     if let Some(mut res) = spotify.playlists().await {

//         for playlist in res.items().iter() {
//             println!("{}", playlist.id());
//             if let Some(mut temp) = spotify.playlist_artists(playlist.id().to_string()).await {
//                 for obj in temp.items.iter() {
//                     for artist in obj.track.artists.iter() {
//                         println!("{}", artist.name());
//                     }
//                 }
//             }
//             // stop
//             return HttpResponse::Ok()
//                 .header("Location", "/result")
//                 .body("done!")
//         }

//         while let Some(page) = spotify.next(&res).await {
//             for play in page.items().iter() {
//                 println!("{}", play.id());
//             }
//             res = page;
//         }
//     }
// }