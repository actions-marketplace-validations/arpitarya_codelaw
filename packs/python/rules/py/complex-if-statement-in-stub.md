---
id: py/complex-if-statement-in-stub
title: "A stub's if tests only sys.version_info or sys.platform"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "what the condition stood for has to be expressed as a version or platform check, or the branches merged"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI002]
since: 0.5.0
---

An `if` in a stub tests a single comparison of `sys.version_info` or `sys.platform`, nothing else.

**Why:** Type checkers evaluate only those comparisons in a stub; any other test is treated as unknown and both branches count.

```codelaw
match:
  kind: if_statement
  has:
    field: condition
    not: {kind: comparison_operator, has: {nthChild: 1, regex: '^sys\.(version_info|platform)'}}
message: "Test only `sys.version_info` or `sys.platform` in a stub"
```

## How to fix

1. Rewrite the test as a `sys.version_info` or `sys.platform` comparison, or drop it and keep the declarations that hold everywhere.

## Examples

### Python

#### Don't

```python path=api.pyi
if os.name == "nt":
    def f() -> None: ...
```

#### Do

```python path=api.pyi
if sys.platform == "win32":
    def f() -> None: ...
```
