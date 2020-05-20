import sys, os
from musixmatch.api import Musixmatch
from musixmatch.models import Artist
from musixmatch.config import MusixmatchTestCase

api_key = os.environ.get('API_KEY', '6cb6ae857f734da1d184085e78d5cb3d')

def searchCountry(artist_name):
    if artist_name:
        musixmatch = Musixmatch(api_key)
        artists = musixmatch.artist.search(artist_name, page_size=3)
        if (artists and artists[0].country):
            return artists[0].country
        else:
            return "unknown"
    else:
        print("Whoops, no artist to search!")
        print("usage: searchCountry(<artist_name>)")
        sys.exit()