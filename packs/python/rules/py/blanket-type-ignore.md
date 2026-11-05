---
id: py/blanket-type-ignore
title: "type: ignore names its error codes"
languages: [python]
fix: manual
fix_reason: "the error codes to name are the type checker's, not in the source"
fix_exception: value
stability: preview
ported_from: [ruff/PGH003]
since: 0.5.0
---

A `# type: ignore` comment names the errors it silences: `# type: ignore[attr-defined]`.

**Why:** A blanket ignore also hides every new error on the line.

```codelaw
match:
  kind: comment
  regex: 'type:\s*ignore\s*($|#)'
message: "Name the error codes: `# type: ignore[code]`"
```

## How to fix

1. Run the type checker, and write the codes it reports: `# type: ignore[attr-defined]`.

## Examples

### Python

#### Don't

```python
x = load()  # type: ignore
```

#### Do

```python
x = load()  # type: ignore[no-untyped-call]
```
