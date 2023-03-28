# -*- coding: utf-8 -*-
"""
=======================
 Configurations Module
=======================

Any constants and abstractions of environment variables go here.
"""
from . import env, logger

HAS_NUMPY: bool = True
"""
Flag to indicate if the current module can use Numpy features.

Set at import time.
"""

HAS_POLARS: bool = True
"""
Flag to indicate if the current module can use Polars features.

Set at import time.
"""


def missing_function_factory(name: str, feature: str):
    """
    A factory function to populate missing functions due to unset flags.
    """

    def _wrapper(*args, **kwargs):
        raise NotImplementedError(
            f"{name} is not available in this installation as `unicode_pyo3` was not "
            f"built with extra of {feature!r}."
        )

    _wrapper.__name__ = name
    return _wrapper
