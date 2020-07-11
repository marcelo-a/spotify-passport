#![allow(unused_variables)]
use std::collections::HashMap;
use serde::Deserialize;
use serde_json;
use actix_web::{
    web, HttpResponse, Responder,
    http::StatusCode
};
use actix_session::Session;
// local crates
use spotify_lib::spotify::api::Passport;
use spotify_lib::musixmatch::musix::Musixmatch;

pub async fn default() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../html/index.html"))
}

pub async fn render_main() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../html/main.html"))
}

pub async fn error() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../html/error.html"))
}

#[derive(Deserialize)]
pub struct Req {
    playlist_id: String,
}

pub async fn test(
    session: Session,
    spotify: web::Data<Passport>,
    url: web::Query<Req>, // deserialize URL query into struct
) -> impl Responder {
    if let Some(token) = session.get::<String>("token").unwrap() {
        let musix = Musixmatch::default();
        if let Some(body) = musix.search_artist(&"Mumford & Sons".to_string()).await {
            let json = serde_json::to_string(&body).unwrap();
            return HttpResponse::Ok()
                    .body(json)
        }
    }
    else {
        println!("token not retrieved");
    }

    HttpResponse::BadRequest()
        .header("Location", "/error")
        .body("Error occurred!")
}

// pub async fn run(req: HttpRequest) -> impl Result<Json<String>> {
pub async fn run(
    session: Session,
    spotify: web::Data<Passport>,
    url: web::Query<Req>, // deserialize URL query into struct
) -> impl Responder {
    if let Some(token) = session.get::<String>("token").unwrap() {

        if let Some(artist_map) = spotify.fetch_artists(&url.playlist_id).await {

            let musix = Musixmatch::default();
            let mut iso_code = HashMap::new();
            let mut freq : HashMap<String, u64> = HashMap::new();
            for artist in artist_map.keys() {
                if let Some(country_code) = musix.search_artist(artist).await {
                    iso_code.insert(artist, country_code.to_owned());
                    if country_code.is_empty() {
                        if freq.contains_key("unknown") {
                            *freq.get_mut("unknown").unwrap() += 1;
                        }
                        else {
                            freq.insert("unknown".to_string(), 0);
                        }
                    }
                    else {
                        if freq.contains_key(&country_code) {
                            *freq.get_mut(&country_code).unwrap() += 1;
                        }
                        else {
                            freq.insert(country_code, 1);
                        }
                    }
                }
            }
            // let json = serde_json::to_string(&freq).unwrap();
            return HttpResponse::Ok()
                    // .body(json)
                    .json(freq)
        }
        else {
            println!("Unable to fetch artists!");
        }
    }
    else {
        println!("token not retrieved");
    }

    HttpResponse::BadRequest()
        .header("Location", "/error")
        .body("Error occurred!")
}

// collect playlists to build form
pub async fn collect_playlists(
    session: Session,
    spotify: web::Data<Passport>,
) -> HttpResponse {
    if let Some(playlist_map) = spotify.retrieve_all_playlists().await {

        return HttpResponse::Accepted()
            .json(playlist_map)
    }
    else {
        println!("Unable to compile list!");
        return error().await
    }
}