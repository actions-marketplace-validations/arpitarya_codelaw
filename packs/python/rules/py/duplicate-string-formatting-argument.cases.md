# py/duplicate-string-formatting-argument — cases

### case: indexed fields, literals, attributes and variable receivers are fine

```py
a = "{0} {0}".format(x)
b = "{} {}".format(1, 1)
c = "{} {}".format(p.q, p.q)
d = fmt.format(x, x)
e = "{} {}".format(x, *rest)
```

```expect
(none)
```

### case: keyword arguments do not count, positional repeats do

```py
a = "{} {k}".format(x, k=x)
b = "{} {} {}".format(x, y, x)
```

```expect
2:5 py/duplicate-string-formatting-argument
```
