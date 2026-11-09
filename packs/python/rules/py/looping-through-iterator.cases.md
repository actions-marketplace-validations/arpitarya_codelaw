# py/looping-through-iterator — cases

### case: a generator expression

```py
def f():
    gen = (x for x in range(3))
    for _i in range(2):
        for item in gen:
            print(item)
```

```expect
4:21 py/looping-through-iterator
```

### case: map, filter, zip, iter and reversed

```py
def f():
    m = map(str, range(3))
    z = zip(range(3), 'abc')
    for _i in range(2):
        for a in m:
            print(a)
        for b in z:
            print(b)
```

```expect
5:18 py/looping-through-iterator
7:18 py/looping-through-iterator
```

### case: through calls in the iterable

```py
def f():
    a = map(str, range(3))
    b = iter(range(3))
    for _i in range(2):
        for x, y in list(zip(a, b)):
            print(x, y)
```

```expect
5:30 py/looping-through-iterator
5:33 py/looping-through-iterator
```

### case: a generator function this file defines

```py
def numbers():
    yield 1


def f():
    gen = numbers()
    for _i in range(2):
        for n in gen:
            print(n)
```

```expect
8:18 py/looping-through-iterator
```

### case: rebuilt in the outer loop

```py
def f():
    for _i in range(2):
        gen = (x for x in range(3))
        for item in gen:
            print(item)
```

```expect
(none)
```

### case: a list and a range

```py
def f():
    a = [x for x in range(3)]
    b = range(3)
    for _i in range(2):
        for x in a:
            print(x)
        for y in b:
            print(y)
```

```expect
(none)
```

### case: the inner loop returns or breaks

```py
def f():
    gen = (x for x in range(3))
    for out in range(3):
        for src in gen:
            if src == out:
                break
            print(src)
```

```expect
(none)
```

### case: the outer loop breaks after the inner one

```py
def f():
    gen = (x for x in range(3))
    for _i in range(3):
        for src in gen:
            print(src)
        break
```

```expect
(none)
```

### case: a break in a loop nested in the inner one is not the inner loop's

```py
def f():
    gen = (x for x in range(3))
    for _i in range(3):
        for src in gen:
            while True:
                break
```

```expect
4:20 py/looping-through-iterator
```

### case: an inner loop under an if

```py
def f():
    gen = (x for x in range(3))
    for i in range(3):
        if i == 0:
            for src in gen:
                print(src)
```

```expect
(none)
```

### case: a single loop

```py
def f():
    gen = (x for x in range(3))
    for item in gen:
        print(item)
```

```expect
(none)
```

### case: an attribute is not a simple name

```py
class C:
    def __init__(self):
        self.it = (x for x in range(3))

    def run(self):
        for i in range(2):
            for x in self.it:
                print(i, x)
```

```expect
(none)
```
