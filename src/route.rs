// #![allow(unused_imports)]
#![allow(unused_variables)]
use std::{env, collections::HashMap};
use serde::Deserialize;
use serde_json;
use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_web::http::{StatusCode};
use actix_session::Session;
use std::time::Instant;
// local crates
use spotify_lib::spotify::api::Passport;
use spotify_lib::musixmatch::musix::Musixmatch;

pub async fn default() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
}

pub async fn render_main() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/main.html"))
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
            // if let Some(country_code) = body.top_result_country() {
            //     println!("{}", country_code);
            // }
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
// ) -> Option<String> {
) -> impl Responder {
    if let Some(token) = session.get::<String>("token").unwrap() {
        // Serialization can fail if T's implementation of Serialize decides to fail, or if T contains a map with non-string keys.
        // println!("{}", url.playlist_id);
        let now = Instant::now();

        if let Some(artist_map) = spotify.fetch_artists(&url.playlist_id).await {
            let musix = Musixmatch::default();
            let mut iso_code = HashMap::new();
            for artist in artist_map.keys() {
                if let Some(country_code) = musix.search_artist(artist).await {
                    println!("{}: {}", artist, country_code);
                    iso_code.insert(artist, country_code);
                }
                else {
                    println!("{}: \"Unknown\"", artist);
                }
            }

            // let then = Instant::now();
            println!("{}", now.elapsed().as_millis());
            let json = serde_json::to_string(&iso_code).unwrap();
            return HttpResponse::Ok()
                    .body(json)
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