from flask import Flask, make_response, request, session, render_template, send_file, Response
from flask.views import MethodView
from datetime import datetime
import humanize
import os
import re
import stat
import json
import mimetypes
import sys
from pathlib2 import Path
from argparse import ArgumentParser


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("--log_path", help="set log server's log path", metavar="FILE")
    return parser.parse_args()


app = Flask(__name__, static_url_path='/assets', static_folder='assets')
root = '.'

datatypes = {'text': 'txt'}
icontypes = {'icon-jfi-file-text': 'txt,log'}


@app.template_filter('size_fmt')
def size_fmt(size):
    return humanize.naturalsize(size)


@app.template_filter('time_fmt')
def time_desc(timestamp):
    mdate = datetime.fromtimestamp(timestamp)
    str = mdate.strftime('%Y-%m-%d %H:%M:%S')
    return str


@app.template_filter('data_fmt')
def data_fmt(filename):
    t = 'unknown'
    for type, exts in datatypes.items():
        if filename.split('.')[-1] in exts:
            t = type
    return t


@app.template_filter('icon_fmt')
def icon_fmt(filename):
    i = 'icon-jfi-file-o'
    for icon, exts in icontypes.items():
        if filename.split('.')[-1] in exts:
            i = icon
    return i


@app.template_filter('humanize')
def time_humanize(timestamp):
    mdate = datetime.utcfromtimestamp(timestamp)
    return humanize.naturaltime(mdate)


def get_type(mode):
    if stat.S_ISDIR(mode) or stat.S_ISLNK(mode):
        type = 'dir'
    else:
        type = 'file'
    return type


class PathView(MethodView):

    def get(self, p=''):

        path = os.path.join(root, p)

        if os.path.isdir(path):

            contents = []
            total = {'size': 0, 'dir': 0, 'file': 0}

            if p:

                stat_res = os.stat('..')

                info = {}
                info['name'] = '..'
                info['mtime'] = stat_res.st_mtime
                ft = get_type(stat_res.st_mode)
                info['type'] = ft
                sz = stat_res.st_size
                info['size'] = sz

                contents.append(info)

            for filename in os.listdir(path):

                filepath = os.path.join(path, filename)
                stat_res = os.stat(filepath)

                info = {}
                info['name'] = filename
                info['mtime'] = stat_res.st_mtime
                ft = get_type(stat_res.st_mode)
                info['type'] = ft
                sz = stat_res.st_size
                info['size'] = sz

                contents.append(info)

                total[ft] += 1
                total['size'] += sz

            page = render_template('index.html', path=p,
                                   total=total, contents=contents)
            return make_response(page, 200)

        else:

            print('path: ', path)

            mime = mimetypes.guess_type(path)
            print('mime: ', mime)

            filetype = mime[0].split('/')[0]
            print('filetype: ', filetype)

            filename = os.path.basename(path)

            if filename == 'favicon.ico':
                return Response('', mimetype='text/plain')

            if filetype == 'text':
                with open(path, 'r') as f:
                    return Response(f.read(), mimetype='text/plain')
            else:
                res = send_file(path)
                res.headers.add('Content-Disposition', 'attachment')
                return res

    def post(self, p=''):
        pass


path_view = PathView.as_view('path_view')
app.add_url_rule('/', view_func=path_view)
app.add_url_rule('/<path:p>', view_func=path_view)


def main():

    global root

    options = parse_args()
    print(options)

    if options.log_path:
        root = options.log_path

    app.run('0.0.0.0', '8000', threaded=True, debug=False)
