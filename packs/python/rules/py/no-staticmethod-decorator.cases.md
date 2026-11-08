# py/no-staticmethod-decorator — cases

### case: the assignment before a decorated method

```py
class Number:
    parse = staticmethod(parse)
    @cache
    def parse(text):
        return int(text)
```

```expect
4:5 py/no-staticmethod-decorator
```

```fixed
class Number:
    @staticmethod
    @cache
    def parse(text):
        return int(text)
```

### case: already decorated, another name, or outside a class

```py
class Number:
    @staticmethod
    def parse(text):
        return int(text)

    parse = staticmethod(parse)
    other = staticmethod(parse)


def parse(text):
    return int(text)


parse = staticmethod(parse)
```

```expect
(none)
```
