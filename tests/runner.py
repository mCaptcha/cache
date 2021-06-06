# Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
# 
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# 
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
from threading import Thread

class Runner(object):
    _functions = []
    _threads = []

    """ Register functions to be run"""
    def register(self, fn):
        self._functions.append(fn)
        t = Thread(target=fn)
        t.start()
        self._threads.append(t)

    """Wait for registered functions to finish executing"""
    def wait(self):
        for thread in self._threads:
            thread.join()

    """Runs in seperate threads"""
    def __init__(self):
        super(Runner, self).__init__()
#        self.arg = arg
