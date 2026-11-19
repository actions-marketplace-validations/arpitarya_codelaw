# py/property-docstring-starts-with-verb — cases

### case: a property that starts with a verb

```py
class A:
    @property
    def a(self):
        """Returns a."""
```

```expect
4:12 py/property-docstring-starts-with-verb
```

### case: each of the ten verbs

```py
class A:
    @property
    def a(self):
        """Return a."""

    @property
    def b(self):
        """get b."""

    @property
    def c(self):
        """Yields c."""

    @property
    def d(self):
        """Fetch d."""

    @property
    def e(self):
        """Retrieves e."""
```

```expect
4:12 py/property-docstring-starts-with-verb
8:12 py/property-docstring-starts-with-verb
12:12 py/property-docstring-starts-with-verb
16:12 py/property-docstring-starts-with-verb
20:12 py/property-docstring-starts-with-verb
```

### case: a noun phrase

```py
class A:
    @property
    def a(self):
        """The a."""
```

```expect
(none)
```

### case: other verbs are not on the list

```py
class A:
    @property
    def a(self):
        """Compute a."""
```

```expect
(none)
```

### case: `cached_property`, imported and qualified

```py
import functools
from functools import cached_property


class A:
    @cached_property
    def a(self):
        """Gets a."""

    @functools.cached_property
    def b(self):
        """Gets b."""
```

```expect
8:12 py/property-docstring-starts-with-verb
12:12 py/property-docstring-starts-with-verb
```

### case: a function that is not a property

```py
def f():
    """Returns the value."""


class A:
    def m(self):
        """Gets the value."""
```

```expect
(none)
```

### case: the first line after leading blank lines

```py
class A:
    @property
    def a(self):
        """

        Returns a.
        """
```

```expect
6:9 py/property-docstring-starts-with-verb
```

### case: a decorator named in `property_decorators`

```py options={"property_decorators":["mylib.prop"]}
from mylib import prop


class A:
    @prop
    def a(self):
        """Returns a."""
```

```expect
7:12 py/property-docstring-starts-with-verb
```

### case: a decorator named in `ignore_decorators`

```py options={"ignore_decorators":["mylib.prop"],"property_decorators":["mylib.prop"]}
from mylib import prop


class A:
    @prop
    def a(self):
        """Returns a."""
```

```expect
(none)
```
