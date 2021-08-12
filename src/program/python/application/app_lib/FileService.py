import os
from threading import local
from logging.config import dictConfig

from flask import Flask, flash
from flask_restful import Api, Resource, reqparse
from werkzeug.datastructures import FileStorage
from werkzeug.utils import secure_filename

from dramatiq import Middleware

from . import config
from . import tasks

log = config.log
broker = tasks.broker

class FlaskMiddleware(Middleware):

    state = local()

    def __init__(self, app):
        self.app = app

    def before_process_message(self, broker, message):
        log.info("before_process_message")
        context = self.app.app_context()
        context.push()
        self.state.context = context

    def after_process_message(self, broker, message, *, result=None, exception=None):
        log.info("after_process_message")
        try:
            context = self.state.context
            context.pop(exception)
            del self.state.context
        except AttributeError:
            pass

    def before_declare_actor(self, broker, actor):
        print(__name__)
        pass

    def after_declare_actor(self, broker, actor):
        print(__name__)
        pass

    after_skip_message = after_process_message


app = Flask(__name__)
api = Api(app)

broker.add_middleware(FlaskMiddleware(app))

class HelloTask(Resource):
    def get(self):
        log.info("start run hello task")
        message = tasks.hello_task.send()
        message.get_result(block=True)
        log.info("end run hello task")
        return "ok", 200


class Upload(Resource):
    def post(self):
        parser = reqparse.RequestParser()
        parser.add_argument('file', type=FileStorage, location='files')
        args = parser.parse_args()
        if 'file' in args and args['file']:
            filename = secure_filename(args['file'].filename)
            log.info(filename)
            args['file'].save(os.path.join(
                '/home/mx/develop/sources/practice/py/flask/upload_file', filename))
            return "ok", 201
        else:
            return "error: No file part", 400

api.add_resource(Upload, '/upload')
api.add_resource(HelloTask, '/hello')
