import spotipy

def getArtists(items, artDict):
    for item in items:
        artist_name = item['track']['artists'][0]['name']
        # print(item['track']['name'])
        if artist_name in artDict:
            artDict[artist_name] += 1
        else:
            artDict[artist_name] = 1

def getUserArtists(username, token, desired):
    sp = spotipy.Spotify(auth=token)
    playlists = sp.user_playlists(username)
    artDict = dict()
    
    for playlist in playlists['items']:
        if playlist['name'] == desired and playlist['owner']['id'] == username:
            results = sp.user_playlist_tracks(user=username, playlist_id=playlist['id'], fields="items,next")
            tracks = results['items']
            getArtists(tracks, artDict)
            while results['next']:
                results = sp.next(results)
                tracks = results['items']
                getArtists(tracks, artDict)
    return sorted(artDict)