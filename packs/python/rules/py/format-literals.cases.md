# py/format-literals — cases

### case: out of order

```py
x = "{1} {0}".format(a, b)
```

```expect
1:5 py/format-literals
```

```fixed
x = "{} {}".format(b, a)
```

### case: a spec and a conversion stay

```py
x = "{0:>5} {1!r}".format(a, b)
```

```expect
1:5 py/format-literals
```

```fixed
x = "{:>5} {!r}".format(a, b)
```

### case: a repeated position, keywords and automatic fields are left alone

```py
x = "{0} {0}".format(a)
y = "{0} {name}".format(a, name=b)
z = "{} {}".format(a, b)
```

```expect
(none)
```
