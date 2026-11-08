# py/if-else-block-instead-of-dict-lookup — cases

### case: a call in a branch is not read

```py
def f(x):
    if x == 1:
        return a()
    elif x == 2:
        return 2
    elif x == 3:
        return 3
```

```expect
(none)
```

### case: two literals is fine

```py
def f(x):
    if x == 1:
        return 1
    elif x == 2:
        return 2
```

```expect
(none)
```
