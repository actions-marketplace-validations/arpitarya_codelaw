# py/no-classmethod-decorator — cases

### case: the assignment before a decorated method

```py
class Number:
    from_text = classmethod(from_text)
    @cache
    def from_text(cls, text):
        return cls(text)
```

```expect
4:5 py/no-classmethod-decorator
```

```fixed
class Number:
    @classmethod
    @cache
    def from_text(cls, text):
        return cls(text)
```

### case: already decorated, another name, or outside a class

```py
class Number:
    @classmethod
    def from_text(cls, text):
        return cls(text)

    from_text = classmethod(from_text)
    other = classmethod(from_text)


def from_text(cls, text):
    return cls(text)


from_text = classmethod(from_text)
```

```expect
(none)
```
