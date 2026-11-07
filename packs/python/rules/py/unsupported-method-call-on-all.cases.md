# py/unsupported-method-call-on-all — cases

### case: `+=` and other lists are fine

```py
__all__ += ["d"]
names.append("x")
mod.__all__.append("x")
```

```expect
(none)
```

### case: extend and remove

```py
__all__.extend(["a", "b"])
__all__.remove("c")
```

```expect
1:1 py/unsupported-method-call-on-all
2:1 py/unsupported-method-call-on-all
```
