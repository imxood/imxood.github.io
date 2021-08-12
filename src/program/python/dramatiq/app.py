from flask import Flask
from flask_dramatiq import Dramatiq

Dramatiq.DEFAULT_BROKER = "dramatiq.brokers.redis:RedisBroker"

app = Flask(__name__)
dramatiq = Dramatiq(app)

@dramatiq.actor()
def my_actor():
    print("run task")

@app.route("/")
def myhandler():
    my_actor.send()
