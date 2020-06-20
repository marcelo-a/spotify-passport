// #![allow(unused_imports)]
use actix_web::{web, App, HttpServer};
use actix_session::CookieSession;
use actix_files::Files as fs;
use oauth2::basic::BasicClient;
// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use spotify_lib::sp_oauth::auth as user;
mod route;

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(route::login))
//             
//     })
//     .bind("127.0.0.1:8888")?
//     .run()
//     .await
// }

#[actix_rt::main]
async fn main() {
    // REPLACE FIELDS WITH YOUR INFO
    user::set_env("SPOTIFY_CLIENT_ID".to_string(),
            "SPOTIFY_CLIENT_SECRET".to_string(),
            "SPOTIFY_REDIRECT_URI".to_string());

    HttpServer::new(|| {
        let client : BasicClient = user::prompt_for_authentication();
        let state = user::AppState::new(client);
        
        App::new()
            .data(state)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
        // serve static files
            .service(
                fs::new("/static", "../static").show_files_listing()
            )
            .route("/", web::get().to(route::default))
        //     .route("/", web::get().to(user::index))
            // authorization
            .route("/login", web::get().to(user::login))
            .route("/logout", web::get().to(user::logout))
            .route("/callback", web::get().to(user::auth))
        //     .route("/callback", web::get().to(route::render_main))
            // test
            .route("/status", web::get().to(user::login_status))
            .route("/home", web::get().to(route::render_main))
            .route("/again", web::get().to(route::again))
            // drivers
            .route("/run", web::get().to(route::run))
    })
    .bind("127.0.0.1:8888")
    .expect("Can not bind to port 8888")
    .run()
    .await
    .unwrap(); // returns std::io::Result<()>
}