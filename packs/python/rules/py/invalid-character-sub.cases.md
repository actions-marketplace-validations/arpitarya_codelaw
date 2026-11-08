# py/invalid-character-sub — cases

### case: in an f-string's text

```py
msg = f"{name}"
```

```expect
1:15 py/invalid-character-sub
```

```fixed
msg = f"{name}\x1a"
```

### case: in a raw string it is left for you

```py
pattern = r"x"
```

```expect
1:14 py/invalid-character-sub
```
