# py/slice-to-remove-prefix-or-suffix — cases

### case: a literal suffix

```py
x = s[:-len(".py")] if s.endswith(".py") else s
```

```expect
1:5 py/slice-to-remove-prefix-or-suffix
```

```fixed
x = s.removesuffix(".py")
```
