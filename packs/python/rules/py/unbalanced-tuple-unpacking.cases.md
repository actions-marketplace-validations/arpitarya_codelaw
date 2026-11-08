# py/unbalanced-tuple-unpacking — cases

### case: a list

```py
[a, b] = [1]
```

```expect
1:1 py/unbalanced-tuple-unpacking
```

### case: a starred target is fine

```py
a, *rest = 1, 2, 3
```

```expect
(none)
```
