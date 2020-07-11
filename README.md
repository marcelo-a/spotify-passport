# spotify-passport
A simple visualization of your Spotify playlists.

## Usage
1. Download or clone the repo.
2. Update personal client\_id, client\_secret, redirect\_uri, musixmatch\_key information in [`main.rs`](src/main.rs).
3. Execute `cargo run` in command line and open `localhost:8888` in browser of choice.
4. Follow the instructions and get to know your listening habits!

### Notes:
Accuracy is known to be an issue and can be attributed to Musixmatch's incomplete artist database. No one's perfect, but it's good enough!

## Design Philosophy
1. [main.rs](src/main.rs)

   This is the Actix app which is responsible for handling routes. Their purposes are listed as comments above their respective names.

2. [route.rs](src/route.rs)

   This file handles most routes and define the behavior for each one. It contains the driver `run()` as well as `collect_playlists()`,
   
   which is responsible for collecting the user's playlists and their IDs to build a dropdown form in the home page.

3. [sp_oauth](src/sp_oauth)

   This module is responsible for handling OAuth2.0 authorization of users.  hold acts as the driver for retrieving and returning all playlist, track, and artist information.

   >_Requires_: Valid client\_id, client\_secret, redirect\_uri environment variables.
   >
   >_Effects_: Authenticates the user and stores an access token for the session (to be used in request to Spotify's web API).

4. [spotify](src/spotify)

   This module fetches the artists in the desired playlist from Spotify's web API.

   > _Requires_: Valid access token.
   >
   > _Effects_: Returns a HashMap with `{key, value}` pair defined by `{artist name, ISO 3166-1 alpha-2}`.

5. [musixmatch](src/musixmatch)

   This module searches for artist nationality information from Musixmatch's web API.

   >_Requires_: Valid Musixmatch API key stored in environment variable.
   >
   > _Effects_:  Returns top search result's country information.

## Built With
+ [Actix](https://actix.rs/)
+ [Spotify Web API](https://developer.spotify.com/documentation/web-api/)
+ [Musixmatch API](https://developer.musixmatch.com/)
+ [Google GeoChart](https://developers.google.com/chart/interactive/docs/gallery/geochart)
