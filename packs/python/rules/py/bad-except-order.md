---
id: py/bad-except-order
title: "A broad handler comes last"
languages: [python]
fix: manual
fix_reason: "which handler goes first depends on what each should catch"
fix_exception: restructure
stability: preview
ported_from: [pylint/bad-except-order]
since: 0.5.0
---

An `except Exception:` (or `BaseException`, or a bare `except:`) handler is the last of its `try`: a handler after it never runs.

**Why:** Handlers are tried in order; the broad one catches what the later one was written for.

```codelaw
match:
  kind: except_clause
  any:
    - regex: '^except\s*:'
    - has: {nthChild: 1, regex: '^(builtins\.)?(Exception|BaseException)$'}
    - has: {kind: as_pattern, has: {nthChild: 1, regex: '^(builtins\.)?(Exception|BaseException)$'}}
  precedes: {kind: except_clause}
message: "A handler after this one never runs"
```

## How to fix

1. Move the narrower handlers above the broad one.

## Examples

### Python

#### Don't

```python
try:
    run()
except Exception:
    log()
except ValueError:
    fix()
```

#### Do

```python
try:
    run()
except ValueError:
    fix()
except Exception:
    log()
```
