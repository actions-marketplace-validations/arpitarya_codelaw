# py/f-string — cases

### case: keywords, an attribute and a conversion

```py
x = "{name}: {0.id!r}".format(user, name=title)
```

```expect
1:5 py/f-string
```

```fixed
x = f"{title}: {user.id!r}"
```

### case: a quote in an argument

```py
x = "{}".format(d["k"])
```

```expect
(none)
```

### case: an unused argument

```py
x = "{}".format(a, b)
```

```expect
(none)
```

### case: a raw string and a spec

```py
x = r"\d{:>3}".format(n)
```

```expect
1:5 py/f-string
```

```fixed
x = rf"\d{n:>3}"
```

### case: a repeated call is not duplicated

```py
x = "{0} {0}".format(f())
```

```expect
(none)
```

### case: a comment inside the call: reported, no edit

```py
x = "{}".format(
    a,  # the value
)
```

```expect
1:5 py/f-string
```
