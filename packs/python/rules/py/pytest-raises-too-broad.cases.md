# py/pytest-raises-too-broad — cases

### case: an empty match is still too broad

```py
with pytest.raises(Exception, match=""):
    run()
```

```expect
1:6 py/pytest-raises-too-broad
```

### case: a narrow exception is not reported

```py
with pytest.raises(KeyError):
    d["x"]
```

```expect
(none)
```
