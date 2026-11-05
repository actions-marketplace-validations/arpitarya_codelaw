---
id: py/shebang-missing-python
title: "A shebang names python"
languages: [python]
fix: manual
fix_reason: "which interpreter the script needs is the author's"
stability: preview
ported_from: [ruff/EXE003]
since: 0.5.0
---

A `#!` line at the top of a Python file runs a Python interpreter.

**Why:** A shebang for another program runs the file with the wrong interpreter.

```codelaw
match:
  kind: comment
  regex: '^#!'
  inside: {kind: module}
  all:
    - not: {regex: 'python'}
  not: {follows: {stopBy: end, regex: '.'}}
message: "This shebang does not run Python"
```

## How to fix

1. Write `#!/usr/bin/env python3`.

## Examples

### Python

#### Don't

```python
#!/bin/bash
print(1)
```

#### Do

```python
#!/usr/bin/env python3
print(1)
```
