import os
import sys
from . import core
from .core import BootstrapProtocol

class Connection(core.Connection):

    def __init__(self, router):
        super().__init__(router)
