use std::env;
use serde::Deserialize;
use actix_session::Session;
use actix_web::http::header;
use actix_web::{web, App, HttpResponse};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl,
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;

pub struct AppState {
    oauth: BasicClient,
}

impl AppState {
    pub fn new(param: BasicClient) -> AppState {
        AppState {
            oauth: param,
        }
    }
}

pub async fn index(session: Session) -> HttpResponse {
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

pub async fn login(data: web::Data<AppState>) -> HttpResponse {
    // Spotify Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = &data
        .oauth
        .authorize_url(CsrfToken::new_random)
        // Requesting read access to user's private playlists.
        .add_scope(Scope::new(
            "playlist-read-private".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Found()
        .header(header::LOCATION, authorize_url.to_string())
        .finish()
}

pub async fn logout(session: Session) -> HttpResponse {
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

pub async fn auth(
    session: Session,
    data: web::Data<AppState>,
    res: web::Query<AuthRequest>, // deserialize URL query into struct
) -> HttpResponse {
    let code = AuthorizationCode::new(res.code.clone());
    let state = CsrfToken::new(res.state.clone());

    // Exchange the code with a token.
    // let token = &data.oauth.exchange_code(code)
    //                 // Set the PKCE code verifier.
    //                 .set_pkce_verifier(pkce_verifier)
    //                 .request(http_client)?;
    let token = &res.code;

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

pub fn prompt_for_authentication() -> BasicClient {
    // Retrieve environment variables
    let spotify_client_id = ClientId::new(
        env::var("SPOTIFY_CLIENT_ID")
            .expect("Missing the CLIENT_ID environment variable."),
    );
    let spotify_client_secret = ClientSecret::new(
        env::var("SPOTIFY_CLIENT_SECRET")
            .expect("Missing the SPOTIFY_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Spotify OAuth2 process.
    let client = BasicClient::new(
        spotify_client_id,
        Some(spotify_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(
        RedirectUrl::new("http://localhost:8888/callback".to_string())
            .expect("Invalid redirect URL"),
    );

    client
}

pub fn setEnv(client_id: String, client_secret: String, redirect_uri: String) {
    env::set_var("SPOTIFY_CLIENT_ID", client_id);
    env::set_var("SPOTIFY_CLIENT_SECRET", client_secret);
    env::set_var("SPOTIFY_REDIRECT_URI", redirect_uri);
}