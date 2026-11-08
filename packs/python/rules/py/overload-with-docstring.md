---
id: py/overload-with-docstring
title: "An @overload has no docstring"
languages: [python]
fix: suggest
fix_reason: "the docstring becomes `...`; its text belongs on the implementation, where it is shown"
stability: preview
ported_from: [ruff/D418]
since: 0.5.0
---

A function marked `@overload` has no docstring: the implementation carries it.

**Why:** Only the implementation's docstring is what `help()` shows; text on an overload is never read.

```codelaw
match:
  kind: expression_statement
  nthChild: 1
  has: {nthChild: 1, kind: string}
  inside:
    kind: block
    inside:
      kind: function_definition
      inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?overload$'}}
message: "Move the docstring to the implementation"
fix: "..."
```

## Examples

### Python

#### Don't

```python
@overload
def get(key: int) -> int:
    """Get by number."""
```

#### Do

```python
@overload
def get(key: int) -> int:
    ...
```
