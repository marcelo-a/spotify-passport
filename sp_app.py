from flask import Flask, jsonify, request, render_template, abort
from urllib import parse
import src.main as passport
import src.auth as sp_auth
from os import environ

app = Flask(__name__)

@app.route('/')
def init():
    
    render_template('main.html')
    return render_template('login.html')

@app.route('/main')
def main():
    return render_template('main.html')

@app.route('/login', methods=['GET', 'POST'])
def login(name=None):
    # cannot see URL parameters in POST method
    # if request.method == 'POST':
    username = request.args.get('username')
    environ['user'] = username

    if username:
        token = sp_auth.auth()
        if token != 'error':
            environ['token'] = token
            response = 'User log in successful.'
            return jsonify(response)
    # else:
        # return 'Wrong method! Use POST.'

@app.route('/run', methods=['GET'])
def run():
    if request.method == 'GET':
        playlist_name = request.args.get('playlist')
        if playlist_name:
            # get args
            playlist_name = parse.unquote(playlist_name) # parse %20 into spaces
            username = environ.get('user')
            token = environ.get('token')

            data = passport.run(username, token, playlist_name)
            return jsonify(data)  # serialize and use JSON headers
        else:
            description='Error! No playlist provided.'
            abort(404, description)
            # return res
    else:
        abort(405)

if __name__ == '__main__':
    app.run(debug=True, port=8888)