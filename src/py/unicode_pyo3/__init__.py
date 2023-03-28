# -*- coding: utf-8 -*-
"""
Python wrapper for unicode-rs crates.

This project includes a Rust binary backend:
- :mod:`lib_unicode_pyo3` which can be loaded as
  :attr:`~unicode_pyo3.bin`.
"""

from . import config
from . import lib_unicode_pyo3 as bin
from .config import logger
from .lib_unicode_pyo3 import segmentate, segmentate_all

try:
    from .lib_unicode_pyo3 import segmentate_all_to_numpy, segmentate_to_numpy
except ImportError:
    config.HAS_NUMPY = False

    segmentate_to_numpy = config.missing_function_factory(
        name="segmentate_to_numpy",
        feature="numpy",
    )

    segmentate_all_to_numpy = config.missing_function_factory(
        name="segmentate_all_to_numpy",
        feature="numpy",
    )


try:
    from .lib_unicode_pyo3 import segmentate_all_to_polars, segmentate_to_polars
except ImportError:
    config.HAS_POLARS = False

    segmentate_to_polars = config.missing_function_factory(
        name="segmentate_to_polars",
        feature="polars",
    )

    segmentate_all_to_polars = config.missing_function_factory(
        name="segmentate_all_to_polars",
        feature="polars",
    )
