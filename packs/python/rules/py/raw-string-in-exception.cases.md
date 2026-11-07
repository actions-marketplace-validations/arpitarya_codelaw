### case: a name `msg` already in the file

```python
def check(msg):
    if not msg:
        raise ValueError("empty message")
```

```expect
3:26 py/raw-string-in-exception
```

```fixed
def check(msg):
    if not msg:
        msg2 = "empty message"
        raise ValueError(msg2)
```
