# py/split-static-string — cases

### case: split on white space

```py
words = "  one  two three ".split()
```

```expect
1:9 py/split-static-string
```

```fixed
words = ["one", "two", "three"]
```

### case: maxsplit by keyword

```py
head = "a,b,c,d".split(",", maxsplit=1)
```

```expect
1:8 py/split-static-string
```

```fixed
head = ["a", "b,c,d"]
```

### case: rsplit

```py
tail = "a,b,c".rsplit(",", 1)
```

```expect
1:8 py/split-static-string
```

```fixed
tail = ["a,b", "c"]
```

### case: white space with a maxsplit keeps the rest whole

```py
rest = " a b c d ".split(maxsplit=2)
```

```expect
1:8 py/split-static-string
```

```fixed
rest = ["a", "b", "c d "]
```

### case: single quotes are kept

```py
x = '1<>2<>3<4'.split('<>')
```

```expect
1:5 py/split-static-string
```

```fixed
x = ['1', '2', '3<4']
```

### case: a raw string stays raw

```py
x = r"a \n b".split()
```

```expect
1:5 py/split-static-string
```

```fixed
x = [r"a", r"\n", r"b"]
```

### case: implicit concatenation

```py
x = ("a " "b").split()
```

```expect
1:5 py/split-static-string
```

```fixed
x = ["a", "b"]
```

### case: a separator that is a name is not read

```py
sep = ","
x = "a,b".split(sep)
```

```expect
(none)
```

### case: an f-string is not read

```py
x = f"{y},b".split(",")
```

```expect
(none)
```

### case: a maxsplit that is not an integer literal

```py
x = "a,b".split(",", maxsplit=n)
y = "a,b".split(",", maxsplit=1.0)
```

```expect
(none)
```

### case: an empty separator is a ValueError

```py
x = "abc".split("")
```

```expect
(none)
```

### case: a comment in the call is lost, so the edit is a suggestion

```py
x = (
    "a,b,c"
    # keep this
    .split(",")
)
```

```expect
2:5 py/split-static-string
```

### case: an empty string

```py
x = "".split()
```

```expect
1:5 py/split-static-string
```

```fixed
x = []
```
