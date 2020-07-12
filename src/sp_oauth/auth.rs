use std::env;
use serde::Deserialize;
use actix_session::Session;
use actix_web::{
    web, HttpResponse, http::{StatusCode, header}
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge,
    RedirectUrl, Scope, TokenUrl, PkceCodeVerifier,
    basic::BasicClient, reqwest::http_client, TokenResponse
};
// other directories
use crate::spotify::api::{Passport};

pub async fn login_status(session: Session) -> HttpResponse {
    let link = if let Some(_login) = session.get::<bool>("login").unwrap() {
        "logout"
    } else {
        "login"
    };

    match session.get::<String>("token") {
        Ok(res) => {
            if let Some(token) = res {
                println!("session token: {}", token);
            }
            else {
                println!("No session token!");
            }
        },
        Err(e) => println!("Error, cannot get session token: {:?}", e),
    }

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

pub async fn prompt_for_authentication(session: Session, data: web::Data<Passport>) -> HttpResponse {
    // if already logged in, skip authorization
    if let Ok(opt) = session.get::<bool>("login") {
        if let Some(_temp) = opt {
            if let Some(_temp) = session.get::<String>("token").unwrap() {
                HttpResponse::Found()
                    .header(header::LOCATION, "/home")
                    .finish()
            }
            else {
                HttpResponse::Found()
                    .header(header::LOCATION, "/")
                    .finish()
            }
        } else {
            // Spotify Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
            // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
            let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
            // data.set_pkce(pkce_code_verifier);
            env::set_var("pkce_verifier", pkce_code_verifier.secret());

            // Generate the authorization URL to which we'll redirect the user.
            let (authorize_url, _csrf_state) = &data
                .oauth()
                .authorize_url(CsrfToken::new_random)
                // Requesting read access to user's private playlists.
                .add_scope(Scope::new(
                    "playlist-read-private".to_string(),
                ))
                // .add_scope(Scope::new(
                //     "user-read-private".to_string(),
                // ))
                .set_pkce_challenge(pkce_code_challenge)
                .url();

            HttpResponse::Found()
                .header(header::LOCATION, authorize_url.to_string())
                .finish()
        }
    }
    else {
        // return error
        return HttpResponse::build(StatusCode::UNAUTHORIZED)
                .content_type("text/html; charset=utf-8")
                .body(include_str!("../../html/error.html"))
    }
}

pub async fn logout(session: Session) -> HttpResponse {
    session.remove("login");
    HttpResponse::Found()
        .header(header::LOCATION, "/")
        .finish()
}

#[derive(Deserialize)]
#[derive(Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn auth(
    session: Session,
    data: web::Data<Passport>,
    res: web::Query<AuthRequest>, // deserialize URL query into struct
) -> HttpResponse {
    // save session token
    let pkce_verifier = env::var("pkce_verifier").unwrap();
    let auth_code = res.code.to_string();
    let token_response = &data.oauth()
                    .exchange_code(AuthorizationCode::new(auth_code))
                    .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
                    .request(http_client)
                    .expect("token request failed");
    
    let token = token_response.access_token().secret();
    // let temp = token.access_token();
    session.set("token", token).unwrap();
    env::set_var("token", token);

    session.set("login", true).unwrap();

    // redirect to /main
    HttpResponse::Found()
        .header(header::LOCATION, "/home")
        .finish()
}

pub fn create_client() -> BasicClient {
    // Retrieve environment variables
    let spotify_client_id = ClientId::new(
        env::var("SPOTIFY_CLIENT_ID")
            .expect("Missing the CLIENT_ID environment variable."),
    );
    let spotify_client_secret = ClientSecret::new(
        env::var("SPOTIFY_CLIENT_SECRET")
            .expect("Missing the SPOTIFY_CLIENT_SECRET environment variable."),
    );
    let spotify_redirect_uri = RedirectUrl::new(
        env::var("SPOTIFY_REDIRECT_URI")
            .expect("Missing the SPOTIFY_REDIRECT_URI environment variable."),
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
        spotify_redirect_uri
            .expect("Invalid redirect URL"),
    );

    client
}

pub fn set_env(client_id: String, client_secret: String, redirect_uri: String, musix_key: String) {
    env::set_var("SPOTIFY_CLIENT_ID", client_id);
    env::set_var("SPOTIFY_CLIENT_SECRET", client_secret);
    env::set_var("SPOTIFY_REDIRECT_URI", redirect_uri);
    env::set_var("MUSIXMATCH_API_KEY", musix_key);
}