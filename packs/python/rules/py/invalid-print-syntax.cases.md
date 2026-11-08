# Cases: py/invalid-print-syntax

### case: a trailing comma

```py
import sys
print >> sys.stderr, "a", "b",
```

```expect
2:1 py/invalid-print-syntax
```

```fixed
import sys
print("a", "b", end=" ", file=sys.stderr)
```
