### Read Me for
# unicode_pyo3

![CI Checks](https://github.com/denwong47/unicode_pyo3/actions/workflows/CI.yml/badge.svg?branch=main)

Python wrapper for unicode-rs crates.

In order to build this package correctly, you'll need to

- have [Rust](https://www.rust-lang.org/learn/get-started) installed,
- have [`git` CLI](https://packages.debian.org/stable/git) installed,
- have [`build-essential`](https://packages.debian.org/sid/build-essential) (Debian) or
  [Xcode Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html#//apple_ref/doc/uid/DTS40014588-CH1-WHAT_IS_THE_COMMAND_LINE_TOOLS_PACKAGE_)
  (macOS)

Then you can simply:

```sh
python3 -m pip install git+https://github.com/denwong47/unicode_pyo3.git
```

### Example Usage
```python
from unicode_pyo3 import segmentate, segmentate_all

_t = (
    "A Unicode Standard Annex (UAX) forms an integral part of "
    "the Unicode Standard, but is published online as a separate "
    "document. The Unicode Standard may require conformance to "
    "normative content in a Unicode Standard Annex, if so specified "
    "in the Conformance chapter of that version of the Unicode "
    "Standard. The version number of a UAX document corresponds to "
    "the version of the Unicode Standard of which it forms a part."
)

assert (
    segmentate(_t)
    == [
        'A Unicode Standard Annex (UAX) forms an integral part of the Unicode Standard, but is published online as a separate document. ',
        'The Unicode Standard may require conformance to normative content in a Unicode Standard Annex, if so specified in the Conformance chapter of that version of the Unicode Standard. ',
        'The version number of a UAX document corresponds to the version of the Unicode Standard of which it forms a part.'
    ]
)

_lines = [
    'Beautiful is better than ugly. Explicit is better than '
        'implicit. Simple is better than complex. Complex is '
        'better than complicated. Flat is better than nested. '
        'Sparse is better than dense. Readability counts.',
    "Special cases aren't special enough to break the "
        "rules. Although practicality beats purity. Errors "
        "should never pass silently. Unless explicitly "
        "silenced.",
    "In the face of ambiguity, refuse the temptation to "
        "guess. There should be one-- and preferably only one "
        "--obvious way to do it. Although that way may not be "
        "obvious at first unless you're Dutch.",
    'Now is better than never. Although never is often better "
        "than *right* now.',
    "If the implementation is hard to explain, it's a bad "
        "idea. If the implementation is easy to explain, it "
        "may be a good idea. Namespaces are one honking great "
        "idea -- let's do more of those!"
]

assert (
    segmentate_all(_lines)
    == [
        [
            'Beautiful is better than ugly. ',
            'Explicit is better than implicit. ',
            'Simple is better than complex. ',
            'Complex is better than complicated. ',
            'Flat is better than nested. ',
            'Sparse is better than dense. ',
            'Readability counts.'
        ],
        [
            "Special cases aren't special enough to break the rules. ",
            'Although practicality beats purity. ',
            'Errors should never pass silently. ',
            'Unless explicitly silenced.'
        ],
        [
            'In the face of ambiguity, refuse the temptation to guess. ',
            'There should be one-- and preferably only one --obvious way to do it. ',
            "Although that way may not be obvious at first unless you're Dutch."
        ],
        [
            'Now is better than never. ',
            'Although never is often better than *right* now.'
        ],
        [
            "If the implementation is hard to explain, it's a bad idea. ",
            'If the implementation is easy to explain, it may be a good idea. ',
            "Namespaces are one honking great idea -- let's do more of those!"
        ]
    ]
)
```
