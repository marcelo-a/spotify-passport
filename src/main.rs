// #![allow(unused_imports)]
use actix_web::{web, middleware, App, HttpServer};
use actix_session::CookieSession;
use actix_files::Files as fs;
use oauth2::basic::BasicClient;
use spotify_lib::sp_oauth::auth as auth;
use spotify_lib::spotify::api as app;
mod route;

#[actix_rt::main]
async fn main() {
    // REPLACE FIELDS WITH YOUR INFO
    auth::set_env("SPOTIFY_CLIENT_ID".to_string(),
            "SPOTIFY_CLIENT_SECRET".to_string(),
            "SPOTIFY_REDIRECT_URI".to_string(),
            "MUSIXMATCH_API_KEY".to_string());

    HttpServer::new(|| {
        let client : BasicClient = auth::create_client();
        let state = app::Passport::new(client);
        
        App::new()
            .data(state)
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::NormalizePath)
        // serve static files
            .service(
                fs::new("/static", "./static")
                .redirect_to_slash_directory()
                .disable_content_disposition()
                // .index_file("/html/index.html")
            )
        // templates/pages
            .route("/", web::get().to(route::default))
            .route("/home", web::get().to(route::render_main))
        // authorization
            .route("/login", web::get().to(auth::prompt_for_authentication))
            .route("/logout", web::get().to(auth::logout))
            .route("/callback", web::get().to(auth::auth))
        // test
            .route("/status", web::get().to(auth::login_status))
            // .route("/playlist", web::get().to(route::test))
            .route("/test", web::get().to(route::test))
        // drivers
            .route("/run", web::get().to(route::run))
            .route("/user_playlists", web::get().to(route::collect_playlists))
        // redirect to error page
            .default_service(
                web::route().to(route::error)
            )
    })
    .bind("127.0.0.1:8888")
    .expect("Can not bind to port 8888")
    .run()
    .await
    .unwrap(); // returns std::io::Result<()>
}