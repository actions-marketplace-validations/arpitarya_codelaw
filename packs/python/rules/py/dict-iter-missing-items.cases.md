# py/dict-iter-missing-items — cases

### case: a dict display

```py
prices = {'tea': 3}
for name, price in prices:
    pass
```

```expect
2:20 py/dict-iter-missing-items
```

```fixed
prices = {'tea': 3}
for name, price in prices.items():
    pass
```

### case: an annotated parameter

```py
def f(prices: dict[str, int]):
    for name, price in prices:
        pass
```

```expect
2:24 py/dict-iter-missing-items
```

```fixed
def f(prices: dict[str, int]):
    for name, price in prices.items():
        pass
```

### case: dict() and a comprehension

```py
a = dict()
b = {k: 1 for k in 'ab'}
for x, y in a:
    pass
for x, y in b:
    pass
```

```expect
3:13 py/dict-iter-missing-items
5:13 py/dict-iter-missing-items
```

### case: keys that are pairs

```py
grid = {(0, 1): 'a'}
for x, y in grid:
    pass
```

```expect
(none)
```

### case: keys annotated as pairs

```py
def f(grid: dict[tuple[int, int], str]):
    for x, y in grid:
        pass
```

```expect
(none)
```

### case: already .items()

```py
prices = {'a': 1}
for name, price in prices.items():
    pass
```

```expect
(none)
```

### case: a list, and a name bound twice

```py
pairs = [(1, 2)]
for a, b in pairs:
    pass
d = {}
d = [(1, 2)]
for a, b in d:
    pass
```

```expect
(none)
```

### case: one target

```py
d = {}
for k in d:
    pass
```

```expect
(none)
```
