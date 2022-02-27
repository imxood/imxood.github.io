import os
from threading import local
from logging.config import dictConfig

from flask import Flask, flash
from flask_restful import Api, Resource, reqparse
from werkzeug.datastructures import FileStorage
from werkzeug.utils import secure_filename

import dramatiq
from dramatiq import Middleware
from dramatiq.brokers.redis import RedisBroker
from dramatiq.encoder import PickleEncoder
from dramatiq.results import Results
from dramatiq.results.backends import RedisBackend


print(__name__)

dictConfig({
    'version': 1,
    'formatters': {'default': {
        'format': '[%(asctime)s] %(levelname)s in %(module)s: %(message)s',
    }},
    'handlers': {'wsgi': {
        'class': 'logging.StreamHandler',
        'stream': 'ext://flask.logging.wsgi_errors_stream',
        'formatter': 'default'
    }},
    'root': {
        'level': 'INFO',
        'handlers': ['wsgi']
    }
})


class FlaskMiddleware(Middleware):

    state = local()

    def __init__(self, app):
        self.app = app

    def before_process_message(self, broker, message):
        print("before_process_message")
        context = self.app.app_context()
        context.push()
        self.state.context = context

    def after_process_message(self, broker, message, *, result=None, exception=None):
        print("after_process_message")
        try:
            context = self.state.context
            context.pop(exception)
            del self.state.context
        except AttributeError:
            pass

    after_skip_message = after_process_message


app = Flask(__name__)
api = Api(app)

encoder = PickleEncoder()
backend = RedisBackend(encoder=encoder)

redis_broker = RedisBroker()
redis_broker.add_middleware(FlaskMiddleware(app))
redis_broker.add_middleware(Results(backend=backend))
dramatiq.set_broker(redis_broker)


@dramatiq.actor(store_results=True)
def hello_task():
    print("Hello, I'm Hello Task")
    return "ok"


class HelloTask(Resource):
    def get(self):
        print("start run hello task")
        result = hello_task.send()
        print("Task Result: " + str(type(result)))
        print("end run hello task")
        return "ok", 200


class Upload(Resource):
    def post(self):
        parser = reqparse.RequestParser()
        parser.add_argument('file', type=FileStorage, location='files')
        args = parser.parse_args()
        if 'file' in args and args['file']:
            filename = secure_filename(args['file'].filename)
            print(filename)
            args['file'].save(os.path.join(
                '/home/mx/develop/sources/practice/py/flask/upload_file', filename))
            return "ok", 201
        else:
            return "error: No file part", 400


api.add_resource(Upload, '/upload')
api.add_resource(HelloTask, '/hello')

if __name__ == "__main__":
    app.run(debug=True)
