# py/wrong-tuple-length-version-comparison — cases

### case: matching lengths, no slice, or an ordering are fine

```py path=api.pyi
import sys
if sys.version_info[:2] == (2, 7): ...
if sys.version_info[:1] == (2,): ...
if sys.version_info == (3,): ...
if sys.version_info[:2] >= (3,): ...
```

```expect
(none)
```

### case: a one-item tuple against two

```py path=api.pyi
import sys
if sys.version_info[:2] != (2,): ...
```

```expect
2:4 py/wrong-tuple-length-version-comparison
```
