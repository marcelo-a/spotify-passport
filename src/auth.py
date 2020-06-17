import spotipy.util as util
from os import environ

def auth():
    # fetch username from environment
    username = environ['user']
    
    token = util.prompt_for_user_token(username)
    
    if token:
        return token
    else:
        return 'error'