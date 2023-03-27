# -*- coding: utf-8 -*-
from unicode_pyo3 import config


def test_env():
    """
    Assert that the PYTEST flag is actually set.
    """
    assert config.env.PYTEST_IS_RUNNING
