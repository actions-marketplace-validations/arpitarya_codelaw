# py/missing-yield-type-doc — cases

### case: an annotation is a type

```py options={"accept_no_yields_doc":false}
from typing import Iterator


def f() -> Iterator[int]:
    """Do it."""
    yield 1
```

```expect
(none)
```

### case: Sphinx, a missing rtype

```py options={"accept_no_yields_doc":false}
def f():
    """Do it.

    :returns: One.
    """
    yield 1
```

```expect
1:5 py/missing-yield-type-doc
```
