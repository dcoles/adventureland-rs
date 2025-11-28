#!/usr/bin/env python3

"""
Local HTTP server for AdventureLand (https://adventure.land).

This can be used as an alternative to always running VSCode.
"""

import argparse
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import logging
import os
import posixpath
import shutil
import urllib

DEFAULT_HOST = '127.0.0.1'
DEFAULT_PORT = 5500


class RequestHandler(BaseHTTPRequestHandler):
    extensions_map = {
        '.html': 'text/html; charset=UTF-8',
        '.txt': 'text/plain',
        '.wgsl': 'text/wgsl',
        '.js': 'text/javascript',
        '.mjs': 'text/javascript',
        '.wasm': 'application/wasm',
    }

    # Browser should never cache responses
    cache_control = 'no_cache'

    # Allow access from adventure.land website
    access_control_allow_origin = 'https://adventure.land'

    cross_origin_opener_policy = 'same-origin'
    cross_origin_embedder_policy = 'require-corp'

    def do_GET(self):
        """Handle GET requests"""
        f = self.send_head()
        if f:
            shutil.copyfileobj(f, self.wfile)
            f.close()

    def do_HEAD(self):
        """Handle HEAD requests"""
        f = self.send_head()
        if f:
            f.close()

    def send_head(self):
        """Common behaviour between GET/HEAD"""
        path = self.translate_path(self.path)
        try:
            f = open(path, 'rb')
        except FileExistsError:
            self.send_error(404, 'Not Found')

        for (suffix, content_type) in self.extensions_map.items():
            if path.endswith(suffix):
                break
        else:
            content_type = 'application/octet-stream'

        st = os.fstat(f.fileno())

        self.send_response(200)
        self.send_header('Content-Type', content_type)
        self.send_header('Content-Length', str(st.st_size))
        if self.access_control_allow_origin:
            self.send_header('Access-Control-Allow-Origin', self.access_control_allow_origin)
        if self.cache_control:
            self.send_header('Cache-Control', self.cache_control)
        if self.cross_origin_opener_policy:
            self.send_header('Cross-Origin-Opener-Policy', self.cross_origin_opener_policy)
        if self.cross_origin_embedder_policy:
            self.send_header('Cross-Origin-Embedder-Policy', self.cross_origin_embedder_policy)
        self.end_headers()

        return f

    def translate_path(self, path):
        """Translate a /-separated PATH to the local filename syntax.

        Components that mean special things to the local file system
        (e.g. drive or directory names) are ignored.  (XXX They should
        probably be diagnosed.)

        """
        # abandon query parameters
        path = path.split('?',1)[0]
        path = path.split('#',1)[0]
        # Don't forget explicit trailing slash when normalizing. Issue17324
        trailing_slash = path.rstrip().endswith('/')
        try:
            path = urllib.parse.unquote(path, errors='surrogatepass')
        except UnicodeDecodeError:
            path = urllib.parse.unquote(path)
        path = posixpath.normpath(path)
        words = path.split('/')
        words = filter(None, words)
        path = os.getcwd()
        for word in words:
            if os.path.dirname(word) or word in (os.curdir, os.pardir):
                # Ignore components that are not a simple file/directory name
                continue
            path = os.path.join(path, word)
        if trailing_slash:
            path += '/'
        return path


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser()
    parser.add_argument('--host', default=DEFAULT_HOST)
    parser.add_argument('--port', type=int, default=DEFAULT_PORT)
    return parser.parse_args()


def main():
    logging.basicConfig(level=logging.INFO)
    args = parse_args()

    logging.info('Listening on http://%s:%d', args.host, args.port)

    with ThreadingHTTPServer((args.host, args.port), RequestHandler) as httpd:
        httpd.serve_forever()


if __name__ == '__main__':
    main()
