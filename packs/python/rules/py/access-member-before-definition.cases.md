# py/access-member-before-definition — cases

### case: a read, then the assignment

```py
class Counter:
    def __init__(self):
        total = self.count
        self.count = 0
        self.total = total
```

```expect
3:17 py/access-member-before-definition
```

### case: an augmented assignment first

```py
class Counter:
    def __init__(self):
        self.count += 5
        self.count = 0
```

```expect
3:9 py/access-member-before-definition
```

### case: the assignment first

```py
class Counter:
    def __init__(self):
        self.count = 0
        total = self.count
        self.total = total
```

```expect
(none)
```

### case: a try that catches AttributeError

```py
class Lazy:
    def __getattr__(self, name):
        try:
            return self.cache
        except AttributeError:
            self.cache = name
            return name
```

```expect
(none)
```

### case: a try that catches something else

```py
class Lazy:
    def get(self, name):
        try:
            return self.cache
        except ValueError:
            self.cache = name
            return name
```

```expect
4:20 py/access-member-before-definition
```

### case: another method assigns it too

```py
class Lazy:
    def reset(self):
        self.cache = None

    def get(self):
        if self.cache:
            return self.cache
        self.cache = 1
```

```expect
(none)
```

### case: a class attribute

```py
class Lazy:
    cache = None

    def get(self):
        if self.cache:
            return self.cache
        self.cache = 1
```

```expect
(none)
```

### case: a Mixin

```py
class LazyMixin:
    def get(self):
        if self.cache:
            return self.cache
        self.cache = 1
```

```expect
(none)
```

### case: a read in a nested function

```py
class Lazy:
    def __init__(self):
        self.callback = lambda: self.later
        self.later = 1
```

```expect
(none)
```

### case: a bare annotation is not an assignment

```py
class Form:
    def process(self):
        _ = self.widget
        self.widget: int
```

```expect
(none)
```

### case: an annotated assignment is

```py
class Form:
    def process(self):
        _ = self.widget
        self.widget: int = 5
```

```expect
3:13 py/access-member-before-definition
```

### case: a method called in __init__ reads it

```py
class Reader:
    def __init__(self):
        self.show()
        self.value = 1

    def show(self):
        return self.value
```

```expect
3:9 py/access-member-before-definition
```

### case: a called method that assigns first

```py
class Reader:
    def __init__(self):
        self.setup()
        self.value = 1

    def setup(self):
        self.value = 0
        return self.value
```

```expect
(none)
```

### case: a call outside __init__

```py
class Reader:
    def reset(self):
        self.show()

    def __init__(self):
        self.value = 1

    def show(self):
        return self.value
```

```expect
(none)
```

### case: a method reference is not a call

```py
class Reader:
    def __init__(self):
        _ = self.show
        self.value = 1

    def show(self):
        return self.value
```

```expect
(none)
```

### case: a base the file imports

```py
from lib import Base


class Child(Base):
    def run(self):
        total = self.count
        self.count = 0
```

```expect
(none)
```
