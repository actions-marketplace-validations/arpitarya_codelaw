# py/ambiguous-unicode-character-comment — cases

### case: curly quotes in a comment

```py
x = 1  # don’t touch
```

```expect
1:13 py/ambiguous-unicode-character-comment
```

### case: a lookalike letter

```py
# Dо not remove
x = 1
```

```expect
1:4 py/ambiguous-unicode-character-comment
```

### case: a string is not a comment

```py
x = "don’t"  # fine
```

```expect
(none)
```
