# -*- coding: utf-8 -*-
"""
Python wrapper for unicode-rs crates.

This project includes a Rust binary backend:
- :mod:`lib_unicode_pyo3` which can be loaded as
  :attr:`~unicode_pyo3.bin`.
"""

from . import lib_unicode_pyo3 as bin
from .config import logger
from .lib_unicode_pyo3 import segmentate, segmentate_all
