# py/ambiguous-unicode-character-string — cases

### case: a lookalike letter in a word with ASCII letters

```py
name = "pаss"
```

```expect
1:10 py/ambiguous-unicode-character-string
```

### case: a curly quote is always reported

```py
msg = "don’t"
```

```expect
1:11 py/ambiguous-unicode-character-string
```

### case: a word of another script is left alone

```py
greeting = "привет мир"
```

```expect
(none)
```

### case: a word of nothing but lookalikes is reported

```py
word = "оо"
```

```expect
1:9 py/ambiguous-unicode-character-string
1:10 py/ambiguous-unicode-character-string
```

### case: a docstring is the docstring rule's

```py
def f():
    """Don’t."""
```

```expect
(none)
```

### case: an f-string is judged outside its fields

```py
x = f"{a} pаss {b}"
```

```expect
1:12 py/ambiguous-unicode-character-string
```

### case: ASCII text is clean

```py
x = "plain"
```

```expect
(none)
```
