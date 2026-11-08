---
id: py/unsupported-method-call-on-all
title: "`__all__` grows with `+=`, not a method call"
languages: [python]
fix: suggest
fix_reason: "`+=` on `__all__` is what static tools read; the author accepts the rewrite, and a `remove` call is theirs to rewrite"
fix_exception: value
stability: preview
ported_from: [ruff/PYI056]
since: 0.5.0
---

`__all__.append("name")`, `__all__.extend([...])` and `__all__.remove("name")` change the export list in a way type checkers and other static tools do not follow. `__all__ += ["name"]` they do. The fix rewrites an `append` or `extend` statement; a `remove` is reported for the author to rewrite.

**Why:** A name added by a method call is exported at run time but not to the tools that read the list without running the module.

```codelaw
match:
  kind: call
  has: {field: function, kind: attribute, regex: '^__all__\.(append|extend|remove)$'}
  any:
    - all:
        - pattern: __all__.append($A)
        - pattern: $APP
        - inside: {kind: expression_statement}
    - all:
        - pattern: __all__.extend($E)
        - pattern: $EXT
        - inside: {kind: expression_statement}
    - regex: '^__all__\.(append|extend|remove)'
message: "Change `__all__` with `+=` so static tools can read it"
fix:
  - replace: { target: $APP, with: "__all__ += [$A]" }
  - replace: { target: $EXT, with: "__all__ += $E" }
```

## Examples

### Python

#### Don't

```python
__all__ = ["load"]
__all__.append("dump")
```

#### Do

```python
__all__ = ["load"]
__all__ += ["dump"]
```
