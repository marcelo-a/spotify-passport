use awc::Client;
use urlencoding::encode;
use regex::Regex;

/* ---------------- Musixmatch API ---------------- */
pub struct Musixmatch {
    root: String,
    api_key: String,
    musix_client: Client,
}

// https://doc.rust-lang.org/std/default/trait.Default.html
impl Default for Musixmatch {
    fn default() -> Musixmatch {
        Musixmatch {
            root: String::from("https://api.musixmatch.com/ws/1.1/"),
            api_key: String::from("MUSIXMATCH_API_KEY"),
            musix_client: Client::default(),
        }
    }
}

impl Musixmatch {
    // search artist
    pub async fn search_artist(&self, artist_name: &String) -> Option<String> {
        let en_name = encode(artist_name);
        let encoded_url = format!(
            // "{}artist.search?apikey={}&q_artist={}&page_size=3&format=json&s_artist_rating=DESC",
            "{0}artist.search?q_artist={2}&page_size=1&format=json&apikey={1}",
            &self.root,
            &self.api_key,
            en_name
        );
            
        match self.musix_client
            .get(encoded_url)
            .header("ContentType", "application/json charset=utf-8")
            .header("Accept", "application/json charset=utf-8")
            .set_header("Accept", "application/json")
            .send().await { // <- send http request and wait for response
                // check if successful
                Ok(mut res) => {
                    if let Ok(body) = res.body().await {
                        let utf = String::from_utf8(body.to_vec()).unwrap();  // .as_ref converts Bytes to [u8]
                        if let Some(artist_country) = parse_country(&utf) {
                            return Some(artist_country)
                        }
                    }
                    return None
                },
                // response error!
                Err(e) => {
                    // panic!("Musixmatch GET request error! {}", e);
                    println!("Musixmatch GET request error! {}", e);
                    return None;
                }
            }
    }

}

pub fn parse_country(body: &String) -> Option<String> {
    let re = Regex::new("\"artist_country\":\"(?P<code>[A-z]*)\"").unwrap();
    if let Some(mat) = re.captures(&body) {
        if let Some(country_code) = mat.name("code") {
            return Some(country_code.as_str().to_string())
        }
        return None
    }
    else {
        return None
    }
}