# py/impossible-comparison — cases

### case: consistent or non-strict pairs, calls and `or` are fine

```py
a = x > y and y < x
b = x >= y and y >= x
c = len(t) > n and n > len(t)
d = x > y or y > x
```

```expect
(none)
```

### case: a non-strict and a strict comparison

```py
e = x >= y and x < y
```

```expect
1:5 py/impossible-comparison
```
