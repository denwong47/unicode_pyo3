# -*- coding: utf-8 -*-
"""
================================
 unicode_pyo3
================================

Python wrapper for unicode-rs crates.

This project includes a Rust binary backend:
- :mod:`lib_unicode_pyo3` which can be loaded as
  :attr:`~unicode_pyo3.bin`.
"""

from . import decorators
from . import lib_unicode_pyo3 as bin
from .config import logger

logger = logger.get(__name__)

logger.info("Welcome to unicode_pyo3!")
logger.debug("Python wrapper for unicode-rs crates.  ")
logger.warning(
    "If you see this message, this means that the Python package was installed "
    "correctly, and __init__.py had run during import."
)
logger.error(
    "This package is currently empty; it does not contain anything of any usefulness."
)
logger.critical("Please populate me!")
