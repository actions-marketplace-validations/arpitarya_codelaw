# py/magic-value-comparison — cases

### case: zero, one, signs of them, strings, two literals and version checks are fine

```py
a = n != -1
b = x != 1.0
c = pw == "hunter2"
d = 10 == 100
e = sys.version_info[0] >= 3
f = x > 0
```

```expect
(none)
```

### case: a negative number and a float

```py
g = argc != -2
h = pi == 3.14159
```

```expect
1:13 py/magic-value-comparison
2:11 py/magic-value-comparison
```
