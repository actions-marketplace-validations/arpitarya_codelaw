# py/unrecognized-version-info-check — cases

### case: the recognised forms and a chain are fine

```py path=api.pyi
import sys
if sys.version_info[0] == 2: ...
if sys.version_info >= (3, 5): ...
if sys.version_info[:1] == (2,): ...
if (2, 7) <= sys.version_info < (3, 5): ...
```

```expect
(none)
```

### case: a list, True, and one side of an `and`

```py path=api.pyi
import sys
if sys.version_info < [3, 0]: ...
if sys.version_info[0] == True: ...
if sys.platform == "linux" and sys.version_info[:3] >= (3, 9): ...
```

```expect
2:4 py/unrecognized-version-info-check
3:4 py/unrecognized-version-info-check
4:32 py/unrecognized-version-info-check
```

