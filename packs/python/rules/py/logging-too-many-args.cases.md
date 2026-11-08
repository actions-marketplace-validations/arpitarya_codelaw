# py/logging-too-many-args — cases

### case: an escaped percent

```py
logging.error("100%% of %s", a, b)
```

```expect
1:1 py/logging-too-many-args
```
