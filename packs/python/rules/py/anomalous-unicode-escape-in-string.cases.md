# py/anomalous-unicode-escape-in-string — cases

### case: str, raw bytes and an escaped backslash are fine

```py
a = "\u00e9"
b = u"\N{BULLET}"
c = rb"\u00e9"
d = b"\\u00e9"
e = b"\x41\n"
```

```expect
(none)
```

### case: beside another escape the finding is manual

```py
f = b"\U00000041\n"
```

```expect
1:5 py/anomalous-unicode-escape-in-string
```

