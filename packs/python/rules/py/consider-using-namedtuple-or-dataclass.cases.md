# py/consider-using-namedtuple-or-dataclass — cases

### case: dicts sharing keys

```py
MAPPING = {
    "entry_1": {"key_1": 0, "key_2": 1, "key_diff_1": 2},
    "entry_2": {"key_1": 0, "key_2": 1, "key_diff_2": 3},
}
```

```expect
1:11 py/consider-using-namedtuple-or-dataclass
```

### case: keys that are constants

```py
class Foo:
    BAR = "bar"


KEY_3 = "key_3"
MAPPING = {
    "entry_1": {KEY_3: None, Foo.BAR: None},
    "entry_2": {KEY_3: None, Foo.BAR: None},
}
```

```expect
6:11 py/consider-using-namedtuple-or-dataclass
```

### case: int keys

```py
MAPPING = {
    "entry_1": {0: None, 1: None},
    "entry_2": {0: None, 1: None},
}
```

```expect
(none)
```

### case: no common keys

```py
MAPPING = {
    "entry_1": {"key_3": 0, "key_4": 1},
    "entry_2": {"key_1": 0, "key_2": 1},
}
```

```expect
(none)
```

### case: in a function

```py
def func():
    mapping = {
        "entry_1": {"key_1": 0, "key_2": 1},
        "entry_2": {"key_1": 0, "key_2": 1},
    }
    return mapping
```

```expect
(none)
```

### case: Final in a function

```py
from typing import Final


def func():
    mapping: Final = {
        "entry_1": {"key_1": 0, "key_2": 1},
        "entry_2": {"key_1": 0, "key_2": 1},
    }
    return mapping
```

```expect
5:22 py/consider-using-namedtuple-or-dataclass
```

### case: lists of one length

```py
MAPPING = {
    "entry_1": [1, "a", set()],
    "entry_2": [2, "b", set()],
}
```

```expect
1:11 py/consider-using-namedtuple-or-dataclass
```

### case: lists of different lengths and empty lists

```py
A = {
    "entry_1": [1],
    "entry_2": [2, "b"],
}
B = {
    "entry_1": [],
    "entry_2": [],
}
```

```expect
(none)
```

### case: tuples

```py
MAPPING = {
    "entry_1": (1, "a"),
    "entry_2": (2, "b"),
}
```

```expect
1:11 py/consider-using-namedtuple-or-dataclass
```

### case: lists of dicts

```py
MAPPING = {
    "entry_1": [{"key_1": None, "key_2": None}],
    "entry_2": [None],
}
```

```expect
(none)
```

### case: scalar values

```py
MAPPING = {
    "entry_1": 1,
    "entry_2": 2,
}
```

```expect
(none)
```
