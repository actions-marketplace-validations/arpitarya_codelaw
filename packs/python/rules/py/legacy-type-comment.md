---
id: py/legacy-type-comment
title: "A stub annotates, not type-comments"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the comment's type moves into an annotation"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI033]
since: 0.5.0
---

A stub writes annotations, not `# type:` comments (a `# type: ignore` is not reported).

**Why:** Stubs are read by tools that support annotations; type comments are the Python 2 form.

```codelaw
match:
  kind: comment
  regex: '^#\s*type:'
  not: {regex: '^#\s*type:\s*ignore'}
message: "Use an annotation instead of a type comment"
```

## How to fix

1. Move the type into an annotation (`x: int`) and delete the comment.

## Examples

### Python

#### Don't

```python path=api.pyi
x = 1  # type: int
```

#### Do

```python path=api.pyi
x: int
```
