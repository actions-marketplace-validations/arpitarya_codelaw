# py/new-line-after-last-paragraph — cases

### case: a one-line docstring is left alone

```py
def f():
    """Return one."""
```

```expect
(none)
```

### case: a method

```py
class A:
    def f(self):
        """One.

        Two."""
```

```expect
3:9 py/new-line-after-last-paragraph
```

```fixed
class A:
    def f(self):
        """One.

        Two.
        """
```
