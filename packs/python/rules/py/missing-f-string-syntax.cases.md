# py/missing-f-string-syntax — cases

### case: a name in scope

```py
def greet(name):
    return "Hello, {name}!"
```

```expect
2:12 py/missing-f-string-syntax
```

```fixed
def greet(name):
    return f"Hello, {name}!"
```

### case: a module name and a format spec

```py
WIDTH = 5
value = 1
line = "{value:>{WIDTH}}"
```

```expect
3:8 py/missing-f-string-syntax
```

```fixed
WIDTH = 5
value = 1
line = f"{value:>{WIDTH}}"
```

### case: a name that is not in scope

```py
def greet():
    return "Hello, {name}!"
```

```expect
(none)
```

### case: `.format` on the literal

```py
def greet(name):
    return "Hello, {name}!".format(name=name)
```

```expect
(none)
```

### case: a docstring and a bare string

```py
def greet(name):
    """Say {name}."""
    "{name}"
```

```expect
(none)
```

### case: gettext and logging

```py
def greet(name):
    _("Hello, {name}")
    logger.info("Hello, {name}")
```

```expect
(none)
```

### case: the name is passed to the call

```py
def greet(name):
    render("Hello, {name}", name=name)
    render("Hello, {name}", name)
```

```expect
(none)
```

### case: a builtin is not a variable in scope

```py
def f():
    return "{id} {type}"
```

```expect
(none)
```

### case: escaped braces and no interpolation

```py
def f(a):
    return "{{a}}", "{}", "{ a b }"
```

```expect
(none)
```

### case: an f-string already

```py
def f(a):
    return f"{a}"
```

```expect
(none)
```
