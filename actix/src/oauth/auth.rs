use std::env;
use serde::Deserialize;
use actix_session::{CookieSession, Session};
use actix_web::http::header;
use actix_web::{web, App, HttpResponse, HttpServer};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};

struct AppState {
    oauth: BasicClient,
}

fn index(session: Session) -> HttpResponse {
    let link = if let Some(_login) = session.get::<bool>("login").unwrap() {
        "logout"
    } else {
        "login"
    };

    let html = format!(
        r#"<html>
        <head><title>OAuth2 Test</title></head>
        <body>
            <a href="/{}">{}</a>
        </body>
    </html>"#,
        link, link
    );

    HttpResponse::Ok().body(html)
}

fn login(data: web::Data<AppState>) -> HttpResponse {
    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new(
            "playlist-read-private".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish()
}

fn logout(session: Session) -> HttpResponse {
    session.remove("login");
    HttpResponse::Found()
        .header(header::LOCATION, "/".to_string())
        .finish()
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

fn auth(
    session: Session,
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> HttpResponse {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());
    // let _scope = params.scope.clone();

    // Exchange the code with a token.
    // let token = &data.oauth.exchange_code(code);
    let token = &params.code;

    session.set("login", true).unwrap();

    let html = format!(
        r#"<html>
        <head><title>OAuth2 Test</title></head>
        <body>
            Spotify returned the following state:
            <pre>{}</pre>
            Spotify returned the following token:
            <pre>{:?}</pre>
        </body>
    </html>"#,
        state.secret(),
        token
    );
    HttpResponse::Ok().body(html)
}

fn setEnv() {
    
}

#[actix_rt::main]
async fn main() {
    setEnv();

    HttpServer::new(|| {
        let google_client_id = ClientId::new(
            env::var("SPOTIFY_CLIENT_ID")
                .expect("Missing the CLIENT_ID environment variable."),
        );
        let google_client_secret = ClientSecret::new(
            env::var("SPOTIFY_CLIENT_SECRET")
                .expect("Missing the SPOTIFY_CLIENT_SECRET environment variable."),
        );
        let auth_url = AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
            .expect("Invalid token endpoint URL");

        // Set up the config for the Google OAuth2 process.
        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_url(
            RedirectUrl::new("http://localhost:8888/callback".to_string())
                .expect("Invalid redirect URL"),
        );

        App::new()
            .data(AppState { oauth: client })
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .route("/", web::get().to(index))
            .route("/login", web::get().to(login))
            .route("/logout", web::get().to(logout))
            .route("/callback", web::get().to(auth))
    })
    .bind("127.0.0.1:8888")
    .expect("Can not bind to port 8888")
    .run()
    .await
    .unwrap();
}