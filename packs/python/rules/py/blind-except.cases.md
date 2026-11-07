# py/blind-except — cases

### case: re-raised or logged

```py
try:
    sync()
except Exception as e:
    raise SyncError() from e
try:
    sync()
except BaseException:
    log.exception("sync failed")
try:
    sync()
except (ValueError, Exception):
    log.error("x", exc_info=True)
```

```expect
(none)
```

### case: a tuple holding Exception

```py
try:
    sync()
except (ValueError, Exception):
    retry()
```

```expect
3:1 py/blind-except
```
