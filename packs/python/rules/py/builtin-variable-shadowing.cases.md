# py/builtin-variable-shadowing — cases

### case: a class attribute is not reported

```py
class A:
    id = 1
```

```expect
(none)
```

### case: a loop target and an except target

```py
def f(xs):
    for id in xs:
        pass
    try:
        pass
    except E as input:
        pass
```

```expect
2:9 py/builtin-variable-shadowing
6:17 py/builtin-variable-shadowing
```

### case: the ignore list

```py options={"ignorelist":["id"]}
def f():
    id = 1
    return id
```

```expect
(none)
```

### case: a public module-level name is reported without a fix

```py
input = 3
```

```expect
1:1 py/builtin-variable-shadowing
```
