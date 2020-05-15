import os
import unittest

from musixmatch.api import Musixmatch

SKIP_TEST = True

api_key = os.environ.get('API_KEY', '6cb6ae857f734da1d184085e78d5cb3d')


class MusixmatchTestCase(unittest.TestCase):

    def setUp(self):
        self.musixmatch = Musixmatch(api_key)
