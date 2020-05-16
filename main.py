import spotipy
import spotipy.util as util
import src.fetchSpotify as spot
import src.fetchCountry as musix
import time

def run(username, playlist_name):
    tic = time.perf_counter()
    if not username or not playlist_name:
        print("Whoops, need your playlist name!")
        return 'error!'

    token = util.prompt_for_user_token(username)

    if token:
        artistDict = spot.getUserArtists(username, token, playlist_name)
        if len(artistDict) == 0:
            return 'Unable to fetch playlist data! Playlist is either empty or private.'
        # countryFreq['unknown'] = 1
        # consider increasing by artist frequency
        countryFreq = dict()
        num = 0
        for artist in artistDict:
            num += 1
            country = musix.searchCountry(artist)
            if country in countryFreq:
                countryFreq[country] += 1
            else:
                countryFreq[country] = 1

        toc = time.perf_counter()
        print("Total runtime: ", toc-tic)
        return countryFreq
        # if "unknown" in countryFreq: print("Unknown: {}, total: {}", countryFreq["unknown"], num)
    else:
        print("Can't get token for", username)
        return "Unable to compile your passport! Please log in."

if __name__ == '__main__':
    run('1289838511', 'Fluorescent Adolescent')