#![allow(unused_imports)]
use std::env;
use actix_web::{web, App, HttpResponse, HttpRequest, Responder};
use actix_web::http::{StatusCode};
// #[macro_use] extern crate serde_derive;

pub async fn default() -> impl Responder {
    // env::set_var("SPOTIFY_CLIENT_ID", "your-spotify-client-id");
    // env::set_var("SPOTIFY_CLIENT_SECRET", "your-spotify-client-secret");
    // env::set_var("SPOTIFY_REDIRECT_URI", "your-app-redirect-uri");

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/index.html"))
}

pub async fn render_main() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../templates/main.html"))
}

pub async fn again() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

// pub async fn run(req: HttpRequest) -> impl Result<Json<String>> {
pub async fn run(req: HttpRequest) -> Option<String> {
    let user_id : String = req.match_info().query("username").parse().unwrap();
    // let user_id = req.query_string().to_string();
    
    // println!("{}", user_id);

    // let client_id = env::var(SPOTIPY_CLIENT_ID);
    // let client_secret = env::var(SPOTIPY_CLIENT_SECRET);
    // let client_id = match env::var("SPOTIPY_CLIENT_ID") {
    //     Ok(val) => val,
    //     Err(e) => println!("couldn't interpret {}: {}", "SPOTIPY_CLIENT_ID", e)
    // };
    let client_secret = match env::var_os("SPOTIPY_CLIENT_SECRET") {
        Some(val) => Some(val),
        _ => None,
    };
    // Some("a".to_string());
    Some(user_id)
}