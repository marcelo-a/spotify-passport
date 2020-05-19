import spotipy.util as util
from os import environ

def auth():
    # fetch username from environment
    username = environ['user']
    # scope = 'playlist-read-private'
    # token = util.prompt_for_user_token(username, scope)
    token = util.prompt_for_user_token(username)
    # token = util.prompt_for_user_token(username,
    #                                     scope,
    #                                     client_id='your-spotify-client-id',
    #                                     client_secret='your-spotify-client-secret',
    #                                     redirect_uri='your-app-redirect-url')
    if token:
        print(token)
        return token
    else:
        print(error)
        return 'error'