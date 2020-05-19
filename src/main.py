import spotipy
import spotipy.util as util
import src.fetchSpotify as spot
import src.fetchCountry as musix
from os import environ

def run(username, token, playlist_name):
    if not username or not playlist_name:
        return 'Whoops, need your username and playlist name!'

    if token:
        artistDict = spot.getUserArtists(username, token, playlist_name)
        
        if len(artistDict) == 0:
            return 'Unable to fetch playlist data! Playlist is either empty or private.'

        countryFreq = dict()
        num = 0
        for artist in artistDict:
            num += 1
            country = musix.searchCountry(artist)
            if country in countryFreq:
                countryFreq[country] += 1
            else:
                countryFreq[country] = 1

        return countryFreq
    else:
        print("Can't get token for", username)
        return "Unable to compile your passport! Please log in."