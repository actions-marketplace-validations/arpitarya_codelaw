# py/consider-using-assignment-expr — cases

### case: a statement between, an attribute, a tuple, an annotation or an elif chain are not read

```py
v = f()
log()
if v:
    pass
A.v = 1
if A.v:
    pass
w = 1, 2
if w:
    pass
z: int = 2
if z:
    pass
k = g()
if k == 1:
    pass
elif k == 2:
    pass
```

```expect
(none)
```

### case: a negated test and a comparison

```py
m = f()
if not m:
    pass
n = g()
if n is None:
    pass
```

```expect
2:1 py/consider-using-assignment-expr
5:1 py/consider-using-assignment-expr
```
