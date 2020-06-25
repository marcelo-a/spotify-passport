// #![allow(unused_imports)]
#![allow(unused_variables)]
use std::env;
use actix_web::{web, App, HttpResponse, HttpRequest, Responder};
use actix_web::http::{StatusCode};
use actix_session::Session;
use serde_json;
// local crates
use spotify_lib::spotify::user::{Passport};


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

pub async fn test(client: web::Data<Passport>) -> impl Responder {
    // client.tracks("58ajLqXikSn2ysmsg2Y4Wq".to_string()).await;
    if let Some(res) = client.playlists().await {
        for playlist in res.items().iter() {
            println!("{}", playlist.id());
        }
        // Serialization can fail if T's implementation of Serialize decides to fail, or if T contains a map with non-string keys.
        let json_struct = serde_json::to_string(&res).unwrap();
        HttpResponse::Ok()
            .body(json_struct)
    }
    else {
        HttpResponse::BadRequest()
            .header("Location", "/error")
            .body("Error occurred!")
    }
}

// pub async fn run(req: HttpRequest) -> impl Result<Json<String>> {
pub async fn run(session: Session, req: HttpRequest) -> Option<String> {
    if let (Ok(client_id), Ok(client_secret)) = (env::var("SPOTIFY_CLIENT_ID"), env::var("SPOTIFY_CLIENT_SECRET")) {
        // let Some(playlist_id) : String = req.match_info().query("playlist").parse();
        // if let Some(token) = session.get::<String>("token");
        // let current_user = user::User;
        // match (current_user, session.get::<String>("token"), req.match_info().query("playlist").parse()) {
        //     (Some(current_user), Ok(token), Some(playlist_id)) => {
        //         if let Some(hashmap) = driver::fetch_info(current_user, token, playlist_id) {
        //             return hashmap;
        //         }
        //         else {
        //             return None;
        //         }
        //     },
        //     _ => return None,
        // }
        return None
    }
    else {
        return None
    }
    // let client_id = match env::var("SPOTIFY_CLIENT_ID") {
    //     Ok(val) => val,
    //     Err(e) => println!("couldn't interpret {}: {}", "SPOTIFY_CLIENT_ID", e)
    // };
    // let client_secret = match env::var_os("SPOTIFY_CLIENT_SECRET") {
    //     Ok(val) => Some(val),
    //     Err(e) => println!("couldn't interpret {}: {}", "SPOTIFY_CLIENT_SECRET", e),
    // };
}