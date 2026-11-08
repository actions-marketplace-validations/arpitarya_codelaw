# py/modified-iterating-set — cases

### case: a discard in the loop

```py
seen = {1, 2, 3}
for n in seen:
    seen.discard(n)
```

```expect
2:1 py/modified-iterating-set
```

```fixed
seen = {1, 2, 3}
for n in seen.copy():
    seen.discard(n)
```

### case: set() and an annotation

```py
a = set()
for n in a:
    a.add(n)


def f(b: set[int]):
    for n in b:
        b.remove(n)
```

```expect
2:1 py/modified-iterating-set
7:5 py/modified-iterating-set
```

### case: another set is changed

```py
a = {1}
b = {2}
for n in a:
    b.add(n)
```

```expect
(none)
```

### case: a copy is iterated

```py
a = {1}
for n in a.copy():
    a.add(n)
```

```expect
(none)
```

### case: a list

```py
a = [1]
for n in a:
    a.append(n)
```

```expect
(none)
```

### case: a read-only call

```py
a = {1}
for n in a:
    a.issubset(a)
```

```expect
(none)
```
