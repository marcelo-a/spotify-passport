# export SPOTIPY_CLIENT_ID='client_id'
# export SPOTIPY_CLIENT_SECRET='client_secret'
# export SPOTIPY_REDIRECT_URI='uri'
# TO REMOVE: unset <var_name>

import spotipy

def getArtists(tracks, artDict):
    for song, item in enumerate(tracks['items']):
        artist_name = item['track']['artists'][0]['name']
        if artist_name in artDict:
            artDict[artist_name] += 1
        else:
            artDict[artist_name] = 1

def getUserArtists(username, token):
    sp = spotipy.Spotify(auth=token)
    playlists = sp.user_playlists(username)
    artDict = dict()
    for playlist in playlists['items']:
        if playlist['owner']['id'] == username:
            results = sp.playlist(playlist['id'], fields="tracks,next")
            tracks = results['tracks']
            getArtists(tracks, artDict)
    # print(sorted(artDict))
    return sorted(artDict)
    # toc = time.perf_counter()
    # print(toc - tic)