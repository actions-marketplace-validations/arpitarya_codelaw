# py/simplify-boolean-expression — cases

### case: a truthy middle, a longer chain, or an `if` test are not read

```py
a = c and True or False
b = c and d and None or e
if c and None or e:
    pass
d = c and x or e
```

```expect
(none)
```

### case: in a return

```py
def f(c, e):
    return c and 0 or e
```

```expect
2:12 py/simplify-boolean-expression
```
