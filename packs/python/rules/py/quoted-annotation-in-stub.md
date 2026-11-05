---
id: py/quoted-annotation-in-stub
title: "No quoted annotations in stubs"
languages: [python]
files: ["**/*.pyi"]
fix: auto
stability: preview
ported_from: [ruff/PYI020]
since: 0.5.0
---

Annotations in a stub are not quoted.

**Why:** Stubs are never executed, so forward references need no quotes.

```codelaw
match:
  kind: string
  regex: '^"[A-Za-z_][\w.\[\], |]*"$'
  has: {kind: string_content, pattern: $C}
  inside: {kind: type}
message: "Remove the quotes"
fix: "$C"
```

## Examples

### Python

#### Don't

```python path=api.pyi
def f(x: "Node") -> None: ...
```

#### Do

```python path=api.pyi
def f(x: Node) -> None: ...
```
