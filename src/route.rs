// #![allow(unused_imports)]
#![allow(unused_variables)]
use std::{env, collections::HashMap};
use serde::Deserialize;
use serde_json;
use actix_web::{web, HttpResponse, HttpRequest, Responder};
use actix_web::http::{StatusCode, header};
use actix_session::Session;
use std::time::Instant;
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
    if let Some(token) = session.get::<String>("token").unwrap() { // panic! for some reason

        let now = Instant::now();

        if let Some(artist_map) = spotify.fetch_artists(&url.playlist_id).await {
            let musix = Musixmatch::default();
            let mut iso_code = HashMap::new();
            let mut freq : HashMap<String, u64> = HashMap::new();
            for artist in artist_map.keys() {
                if let Some(country_code) = musix.search_artist(artist).await {
                    // println!("{}: {}", artist, country_code);
                    iso_code.insert(artist, country_code.to_owned());
                    if country_code.is_empty() {
                        if freq.contains_key("unknown") {
                            *freq.get_mut("unknown").unwrap() += 1;
                        }
                        else {
                            freq.insert("unknown".to_string(), 0); // init unk
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
            println!("{:?}", freq);
            // let then = Instant::now();
            println!("{}", now.elapsed().as_millis());
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

pub async fn collect_playlists(
    session: Session,
    spotify: web::Data<Passport>,
) -> HttpResponse {
    // collect playlists to build form
    if let Some(playlist_map) = spotify.retrieve_all_playlists().await {
        // redirect to /main and send names
        // let json_map = serde_json::to_string(&playlist_map).unwrap();

        return HttpResponse::Accepted()
            // .header(header::ContentType, "application/json")
            .json(playlist_map)
            // .body(json_map)
    }
    else {
        println!("Unable to compile list!");
        return error().await
    }
}