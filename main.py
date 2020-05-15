import sys, spotipy, time
import spotipy.util as util
import src.fetchSpotify as spot
import src.fetchCountry as musix

if __name__ == "__main__":
    tic = time.perf_counter()
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
            # print(artist)
            country = musix.searchCountry(artist)
            if country in countryFreq:
                countryFreq[country] += 1
            # elif country == '':
                # countryFreq['unknown'] += 1
            else:
                countryFreq[country] = 1

        print(countryFreq)
        print("Unkown: {}, total: {}", countryFreq["unknown"], num)
    else:
        print("Can't get token for", username)
    
    toc = time.perf_counter()
    print("Total runtime: ", toc-tic)
