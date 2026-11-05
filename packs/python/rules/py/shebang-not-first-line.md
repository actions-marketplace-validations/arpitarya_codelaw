---
id: py/shebang-not-first-line
title: "The shebang is the first line"
languages: [python]
fix: auto
fix_reason: "a shebang not on line 1 is a plain comment; moving it is safe"
stability: preview
ported_from: [ruff/EXE005]
since: 0.5.0
---

A `#!` comment appears only on the first line.

**Why:** On any other line the kernel ignores it, so the script is not run the way it says.

```codelaw
match:
  kind: comment
  regex: '^#!'
  inside: {kind: module}
  follows: {stopBy: end, regex: '.'}
message: "A shebang only works on the first line"
fix:
  - move-to: start-of-file
```

## Examples

### Python

#### Don't

```python
# helper
#!/usr/bin/env python3
print(1)
```

#### Do

```python
#!/usr/bin/env python3
# helper
print(1)
```
