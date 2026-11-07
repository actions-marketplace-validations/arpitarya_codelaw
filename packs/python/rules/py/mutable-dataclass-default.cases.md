### case: a value with items is built by a lambda, the import joined

```python
from dataclasses import dataclass


@dataclass
class Cart:
    items: list = [1, 2]
```

```expect
6:5 py/mutable-dataclass-default
```

```fixed
from dataclasses import dataclass, field


@dataclass
class Cart:
    items: list = field(default_factory=lambda: [1, 2])
```
