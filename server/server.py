from flask import Flask, request
app = Flask(__name__, static_folder='static', static_url_path="/static")

@app.route('/projects', methods=['POST', 'GET'])
def post_projects():
    return "new a project"

@app.route('/projects', methods=['GET'])
def get_projects():
    return "GET all projects info"

@app.route('/projects/<projectID>', methods=['GET'])
def get_unique_project(projectID):
    return "get an unique projects info"

@app.route('/projects/<projectID>', methods=['DELETE'])
def delete_unique_project(projectID):
    return "delete a project"

@app.route('/projects/<projectID>/submissions', methods=['POST'])
def post_submissions(projectID):
    return "submit"

@app.route('/projects/<projectID>/submissions', methods=['GET'])
def get_submissions(projectID):
    return "get all submissions"

@app.route('/projects/<projectID>/submissions/<submissionID>', methods=['GET'])
def get_unique_submission(projectID, submissionID):
    return f"get an unique projects {submissionID}"


@app.route('/<path:subpath>')
def response_static_file(subpath):
    return app.send_static_file(subpath)

@app.route('/', defaults={'path': ''})
def index(path):
    return app.send_static_file("index.html")

if __name__ == '__main__':
    app.run(host="0.0.0.0")