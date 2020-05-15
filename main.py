import sys, spotipy
# import time
import spotipy.util as util
import src.fetchSpotify as spot
import src.fetchCountry as musix

def run(username):
    # tic = time.perf_counter()
    if len(sys.argv) > 1:
        username = sys.argv[1]
    else:
        print("Whoops, need your username!")
        print("usage: python user_playlists.py [username]")
        sys.exit()

    token = util.prompt_for_user_token(username)

    if token:
        artistDict = spot.getUserArtists(username, token)
        countryFreq = dict()
        # countryFreq['unknown'] = 1
        # consider increasing by artist frequency
        num = 0
        for artist in artistDict:
            num += 1
            country = musix.searchCountry(artist)
            if country in countryFreq:
                countryFreq[country] += 1
            else:
                countryFreq[country] = 1

        print(countryFreq)
        return countryFreq
        # if "unknown" in countryFreq: print("Unknown: {}, total: {}", countryFreq["unknown"], num)
    else:
        print("Can't get token for", username)
        return "Unable to compile your passport!"
    
    # toc = time.perf_counter()
    # print("Total runtime: ", toc-tic)
