# py/invalid-character-esc — cases

### case: in an f-string's text

```py
msg = f"{name}"
```

```expect
1:15 py/invalid-character-esc
```

```fixed
msg = f"{name}\x1b"
```

### case: in a raw string it is left for you

```py
pattern = r"x"
```

```expect
1:14 py/invalid-character-esc
```
