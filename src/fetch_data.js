 /* globals Spotify */
 // client-side js
 // run by the browser each time your view template is loaded
 
 var user;
 
 const userAccessToken = require('../auth.js');
 // const LOCALSTORAGE_ACCESS_TOKEN_EXPIRY_KEY = 'spotify-audio-analysis-playback-token-expires-in';
 // const accessToken = localStorage.getItem(LOCALSTORAGE_ACCESS_TOKEN_KEY);
 
window.onload = () => {
    alert(userAccessToken.token);
    const input = document.querySelector('input');
    // add routine for submit button
    document.querySelector('form').addEventListener('submit', getPlaylistTracks);
};

function name(event) {
  fetch(searchQuery).then(response => response.json()).then(
      buildArtistArray(response)
  ).catch(error => {
      document.getElementById('results').innerHTML = error;
  });
}

function getPlaylistTracks(event) {
    event.preventDefault();
    // const input = document.querySelector('input');
    // let query = `https://api.spotify.com/v1/playlists/${input.value}/tracks`;
    var uri = '6KOuLX5I2jtJqgkloDcCot';
    let query = `https://api.spotify.com/v1/playlists/${uri}/tracks`;

    fetch(query, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${userAccessToken}`     
        }
    }).then(response => response.json())
        .then(({data}) => {
            console.log(data.json());
            // drawAnalysis(data);
            // fetch(`https://api.spotify.com/v1/me/user/${query}`, {
                // method: "PUT",
                // body: JSON.stringify({"uris": [`spotify:track:${id}`]}),
                // headers: {
                // 'Authorization': `Bearer ${accessToken}`
                // }
            // }).catch(e => console.error(e));
            alert('fetched');
        })
        .catch(err => console.log(err));
}

function onSpotifyuserAPIReady() {
  user = new Spotify.user({
    name: 'Passport user',
    getOauthToken: function (callback) { callback(accessToken); },
    volume: 0.8
  });
  
  // Ready
  user.on('ready', function (data) {
    deviceId = data.device_id;
    setTimeout(() => {
      fetch('https://api.spotify.com/v1/me/user', {
        method: "PUT",
        body: JSON.stringify({
          device_ids:[
            data.device_id
          ],
          play: false
        }),
        headers: {
          'Authorization': `Bearer ${accessToken}`
        }
      }).catch(e => console.error(e));
    }, 100);
  });
  // Connect to the user!
  user.connect();
}

// document.addEventListener('DOMContentLoaded', () => {
// const input = document.querySelector('input');
// document.querySelector('form').addEventListener('submit', function(event) {
//     event.preventDefault();
//     let query = 'playlists/{' + input.value + '}/tracks';
//     alert(query);

//     return fetch(query).then(e => e.json()).then(data => {
//         // drawAnalysis(data);
//         fetch(`https://api.spotify.com/v1/me/user/${query}`, {
//             method: "PUT",
//             body: JSON.stringify({"uris": [`spotify:track:${id}`]}),
//             headers: {
//             'Authorization': `Bearer ${accessToken}`
//             }
//         }).catch(e => console.error(e));
//     })
//     });
// });