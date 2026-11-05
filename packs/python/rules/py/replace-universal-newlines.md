---
id: py/replace-universal-newlines
title: "text=, not universal_newlines="
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP021]
since: 0.5.0
---

`subprocess` calls pass `text=True`, not `universal_newlines=True`.

**Why:** `text` is the current name for the same option.

```codelaw
match:
  kind: keyword_argument
  all:
  - has: {field: name, regex: ^universal_newlines$}
  - has: {field: value, pattern: $V}
  - inside:
      kind: argument_list
      inside:
        kind: call
        has: {field: function, regex: ^subprocess\.(run|Popen|call|check_call|check_output)$}
message: Write `text=$V`
fix: text=$V
```

## Examples

### Python

#### Don't

```python
out = subprocess.run(cmd, universal_newlines=True)
```

#### Do

```python
out = subprocess.run(cmd, text=True)
```
