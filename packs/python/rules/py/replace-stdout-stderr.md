---
id: py/replace-stdout-stderr
title: "capture_output=True, not stdout=PIPE, stderr=PIPE"
languages: [python]
fix: suggest
fix_reason: "the same pipes; the keywords move"
stability: preview
ported_from: [ruff/UP022]
since: 0.5.0
---

`subprocess.run(..., stdout=PIPE, stderr=PIPE)` is written `subprocess.run(..., capture_output=True)`.

**Why:** `capture_output` says it directly.

```codelaw
match:
  pattern: subprocess.run($C, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
message: "Write `capture_output=True`"
fix: "subprocess.run($C, capture_output=True)"
```

## Examples

### Python

#### Don't

```python
out = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
```

#### Do

```python
out = subprocess.run(cmd, capture_output=True)
```
