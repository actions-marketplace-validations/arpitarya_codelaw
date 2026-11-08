# py/missing-parentheses-for-call-in-test — cases

### case: a call, a negation, or a plain variable are fine

```py
def ready():
    return True

flag = True
if ready():
    pass
if not ready:
    pass
if flag:
    pass
```

```expect
(none)
```

### case: a conditional expression and a comprehension

```py
def ready():
    return True

x = 1 if ready else 2
y = [i for i in items if ready]
```

```expect
4:10 py/missing-parentheses-for-call-in-test
5:26 py/missing-parentheses-for-call-in-test
```
