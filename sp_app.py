from flask import Flask, jsonify, request, render_template
from urllib import parse
import main as passport
app = Flask(__name__)

@app.route('/')
def home(name=None):
    print('Render home page')
    return render_template('main.html', name=name)

@app.route('/login', methods=['GET', 'POST'])
def login():
    if request.method == 'POST':
        username = request.args.get('username')
        if username:
            return 'SUCCESSFULLY LOGGED IN'
        else:
            return 'UNABLE TO LOG IN!'
        return 'Incoming..'
    else:
        return 'Wrong method! Use POST.'

@app.route('/run', methods=['GET'])
def run():
    if request.method == 'GET':
    # GET request
        username = str(request.args.get('username'))
        playlist_name = request.args.get('playlist')
        if username and playlist_name:
            playlist_name = parse.unquote(playlist_name) # parse %20 into spaces
            data = passport.run(username, playlist_name)
            return jsonify(data)  # serialize and use JSON headers
        else:
            return 'run() error! No username provided.'
    else:
        return 'run() error!'

if __name__ == '__main__':
    app.run(debug=True, port=8888)