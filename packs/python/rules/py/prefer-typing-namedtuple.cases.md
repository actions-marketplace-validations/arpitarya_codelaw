# py/prefer-typing-namedtuple — cases

### case: a local function of the same name is not read

```py
def namedtuple(name, fields):
    return name

P = namedtuple("P", "x y")
```

```expect
(none)
```

### case: the module attribute, in a class's bases

```py
import collections

class C(collections.namedtuple("C", "x y")):
    pass
```

```expect
3:9 py/prefer-typing-namedtuple
```
