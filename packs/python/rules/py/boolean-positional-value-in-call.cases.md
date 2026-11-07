# py/boolean-positional-value-in-call — cases

### case: allowed names, keywords and one-argument setters are fine

```py
d.get("k", False)
os.set_blocking(0, False)
widget.setVisible(True)
widget.set_visible(True)
Value("b", True)
f(x=True)
```

```expect
(none)
```

### case: a setter with two positional arguments, and both flags

```py
widget.set_size(10, True)
deploy(True, False)
```

```expect
1:21 py/boolean-positional-value-in-call
2:8 py/boolean-positional-value-in-call
2:14 py/boolean-positional-value-in-call
```
