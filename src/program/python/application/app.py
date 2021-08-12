from app_lib import config
from app_lib import tasks

log = config.log


if __name__ == "__main__":
    # message = tasks.task_1.send()
    # res = message.get_result(block=True)
    # log.info(res)
    uri = "http://example.com"

    """ pipe = tasks.pipeline([
        tasks.get_uri_contents.message(uri),
        tasks.count_words.message(uri),
    ]).run() """

    pipe = tasks.get_uri_contents.message(uri) | tasks.count_words.message(uri)

    pipe.run()

    for res in pipe.get_results(block=True):
        log.info(res)

    # message = tasks.get_uri_contents.send(uri)
    # res = message.get_result(block=True)
    # log.info(res)
