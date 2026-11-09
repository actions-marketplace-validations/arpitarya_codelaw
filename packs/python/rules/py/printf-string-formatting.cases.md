# py/printf-string-formatting — cases

### case: a precision

```py
x = "%.2f" % total
```

```expect
1:5 py/printf-string-formatting
```

```fixed
x = "{:.2f}".format(total)
```

### case: a mapping

```py
x = "%(a)s-%(b)s" % {"a": 1, "b": 2}
```

```expect
1:5 py/printf-string-formatting
```

```fixed
x = "{a}-{b}".format(a=1, b=2)
```

### case: %d has no equivalent: reported, no edit

```py
x = "%d items" % count
```

```expect
1:5 py/printf-string-formatting
```

### case: a one-element tuple

```py
x = "%s" % (a,)
```

```expect
1:5 py/printf-string-formatting
```

```fixed
x = "{}".format(a)
```

### case: a literal percent

```py
x = "100%% of %s" % name
```

```expect
1:5 py/printf-string-formatting
```

```fixed
x = "100% of {}".format(name)
```

### case: a modulo of numbers is not a format

```py
x = a % b
```

```expect
(none)
```
