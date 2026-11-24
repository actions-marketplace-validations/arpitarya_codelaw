# py/non-imperative-mood — cases

### case: a verb in the third person

```py
def f():
    """Returns the value."""
```

```expect
2:5 py/non-imperative-mood
```

### case: the imperative form

```py
def f():
    """Return the value."""
```

```expect
(none)
```

### case: the -ing and -ed forms of a listed verb

```py
def f():
    """Adding the value."""


def g():
    """Added the value."""


def h():
    """Returned the value."""
```

```expect
6:5 py/non-imperative-mood
10:5 py/non-imperative-mood
```

### case: a blacklisted first word

```py
def f():
    """This returns the value."""


def g():
    """The value."""
```

```expect
2:5 py/non-imperative-mood
6:5 py/non-imperative-mood
```

### case: the first line is the first non-blank line

```py
def f():
    """
    Returns the value.
    """
```

```expect
2:5 py/non-imperative-mood
```

### case: a word the lists do not judge

```py
def f():
    """Frobnicates the value."""


def g():
    """Widget for the value."""
```

```expect
(none)
```

### case: punctuation and capitals around the word are ignored

```py
def f():
    """'RETURNS' the value."""
```

```expect
2:5 py/non-imperative-mood
```

### case: a method

```py
class A:
    def f(self):
        """Gets the value."""
```

```expect
3:9 py/non-imperative-mood
```

### case: a test function is not read

```py
def test_value():
    """Returns the value."""


def runTest():
    """Returns the value."""
```

```expect
(none)
```

### case: a property is not read

```py
from functools import cached_property


class A:
    @property
    def a(self):
        """Returns a."""

    @cached_property
    def b(self):
        """Returns b."""
```

```expect
(none)
```

### case: a setter is read

```py
class A:
    @a.setter
    def a(self, value):
        """Sets the value."""
```

```expect
4:9 py/non-imperative-mood
```

### case: a function with no docstring, or an empty one

```py
def f():
    pass


def g():
    """"""
```

```expect
(none)
```

### case: a class and a module are not read

```py
"""Returns nothing."""


class A:
    """Returns nothing."""
```

```expect
(none)
```

### case: a decorator named in `property_decorators`

```py options={"property_decorators":["mylib.cached"]}
from mylib import cached


@cached
def f():
    """Returns the value."""
```

```expect
(none)
```

### case: a decorator named in `ignore_decorators`

```py options={"ignore_decorators":["mylib.cached"]}
from mylib import cached


@cached
def f():
    """Returns the value."""
```

```expect
(none)
```
