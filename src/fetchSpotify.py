# export SPOTIPY_CLIENT_ID='client_id'
# export SPOTIPY_CLIENT_SECRET='client_secret'
# export SPOTIPY_REDIRECT_URI='uri'
# TO REMOVE: unset <var_name>

# VERION 1
# import spotipy, time

# def getArtists(tracks, artDict):
#     for song, item in enumerate(tracks['items']):
#         artist_name = item['track']['artists'][0]['name']
#         if artist_name in artDict:
#             artDict[artist_name] += 1
#         else:
#             artDict[artist_name] = 1

# def getUserArtists(username, token):
#     sp = spotipy.Spotify(auth=token)
#     playlists = sp.user_playlists(username)
#     # desired = input("Enter your desired playlist name: ")
#     desired = "BONEYARD"
#     tic = time.perf_counter()
#     artDict = dict()
#     for playlist in playlists['items']:
#         if playlist['name'] == desired and playlist['owner']['id'] == username:
#             results = sp.playlist(playlist['id'], fields="tracks, next")
#             tracks = results['tracks']
#             getArtists(tracks, artDict)
#             while tracks['next']:
#                 tracks = sp.next(tracks)
#                 getArtists(tracks, artDict)
#     return sorted(artDict)

# VERSION 2
import spotipy, time

def getArtists(items, artDict):
    for item in items:
        artist_name = item['track']['artists'][0]['name']
        # print(item['track']['name'])
        if artist_name in artDict:
            artDict[artist_name] += 1
        else:
            artDict[artist_name] = 1

def getUserArtists(username, token):
    sp = spotipy.Spotify(auth=token)
    playlists = sp.user_playlists(username)
    # desired = input("Enter your desired playlist name: ")
    desired = "BONEYARD"
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