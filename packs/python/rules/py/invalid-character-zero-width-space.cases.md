# py/invalid-character-zero-width-space — cases

### case: in an f-string's text

```py
msg = f"{name}​"
```

```expect
1:15 py/invalid-character-zero-width-space
```

```fixed
msg = f"{name}\u200b"
```

### case: in a raw string it is left for you

```py
pattern = r"x​"
```

```expect
1:14 py/invalid-character-zero-width-space
```
