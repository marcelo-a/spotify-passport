# spotify-passport
A simple visualization of your Spotify playlists.

## Usage
This repo is currently under construction and is not in a usable state.

### Intended use:
Simply log in to Spotify and paste in your desired playlist's URI.
The URI can be found by right clicking the desired playlist, then `Share` > `Copy Spotify URI`.

## Desgin Philosophy
1. **fetch_data.js**

   Upon submitting the URI, `fetch_data.js` will build a string in the correct Spotify Web API format.
   Subsequently, a `GET` request will be sent to Spotify and will return a json file with information on the all songs in the playlist. The program will then build an array of all unique artists in the playlist.

2. **aggregate.js**

   Once a list of artists is compiled, `aggregate.js` will interate over each artist and send a `GET` request to Musixmatch which will in turn provide another json file. This file will be parsed for each artist's respective country of origin. This will be stored in a map with `{key, value}` pair defined by `{country, frequency}`.

3. **build_map.js**

   At the final step, `build_map.js` will convert the country/frequency map to a data table and then fed to `geochart.js` as a parameter to `google.visualization.arrayToDataTable()`. A world map displaying the countries represented in the playlist and number of artists belonging to such country will be beautifully rendered.

## Built With
+ [Google GeoChart](https://developers.google.com/chart/interactive/docs/gallery/geochart)
+ [Spotify Web API](https://developer.spotify.com/documentation/web-api/)
+ [Musixmatch API](https://developer.musixmatch.com/)