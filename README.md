# spotify-passport
A simple visualization of your Spotify playlists.

## Usage
1. Download or clone the repo.
2. Update personal client\_id, client\_secret, redirect\_uri information in [`sp_app.py`](sp_app.py).
3. Run `python3 sp_appy.py` and open `localhost:8000` in browser of choice.
4. Follow the instructions and get to know your listening habits!

### Notes:
Your Spotify URI can be found by right clicking your account name, then `Share` > `Copy Spotify URI`.

## Design Philosophy
1. [sp_app.py](sp_app.py)

   This is the Python Flask app which is responsible for handling routes.
   Upon submitting the URI, [`auth.py`](src/auth.py) will authorize the log in and retrieve a session token.
   Upon a successful log in, the user will be redirected to the main page, [`main.html`](templates/main.html).

2. [main.py](src/main.py)

   This file acts as the driver for retrieving and returning all playlist, track, and artist information.

   >_Requires_: Valid username, playlist name, and access tokens.
   >
   >_Effects_: Returns a dictionary with `{key, value}` pair defined by `{country, artist_frequency}`.
   
3. [auth.py](src/auth.py)

   This script authenticates the user and return an access token for the session.

   > _Requires_: Environment variable 'user'.
   >
   > _Effects_: Returns valid access token.

4. [fetchSpotify.py](src/fetchSpotify.py)

   This script fetches the artists in the desired playlist from Spotify's web API.

   > _Requires_: Valid username, access token, and desired playlist name.
   >
   > _Effects_: Returns a sorted dictionary with `{key, value}` pair defined by `{artist, artist_frequency}`.

5. [fetchCountry.py](src/fetchCountry.py)

   This script searches for artist nationality information from Musixmatch's web API.

   >_Requires_: Valid artist name.
   >
   > _Effects_:  Returns top search result's country information. Calls `sys.exit()` if artist_name is null.


## Built With
+ [Spotipy](https://spotipy.readthedocs.io/en/2.12.0/)
+ [Spotify Web API](https://developer.spotify.com/documentation/web-api/)
+ [Musixmatch API](https://developer.musixmatch.com/)
+ [yakupadakli's Python wrapper to Musixmatch's API](https://github.com/yakupadakli/python-musixmatch)
+ [Google GeoChart](https://developers.google.com/chart/interactive/docs/gallery/geochart)