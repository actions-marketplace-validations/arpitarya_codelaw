---
id: py/blind-except
title: "No except Exception that swallows the error"
languages: [python]
fix: manual
fix_reason: "which exceptions the handler should catch is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/BLE001, pylint/broad-exception-caught]
since: 0.5.0
---

An `except Exception:` or `except BaseException:` handler (alone or in a tuple) either re-raises — a bare `raise`, `raise e` or `raise … from e` — or logs the traceback with `.exception(...)` or `exc_info=`. Otherwise it is reported.

**Why:** A broad handler that does neither hides every error, including the bugs it was not written for.

```codelaw
utils:
  broad:
    any:
      - {kind: identifier, regex: '^(builtins\.)?(Exception|BaseException)$'}
      - {kind: attribute, regex: '^(builtins\.)?(Exception|BaseException)$'}
      - {kind: tuple, has: {any: [{kind: identifier}, {kind: attribute}], regex: '^(builtins\.)?(Exception|BaseException)$'}}
match:
  kind: except_clause
  any:
    - has: {matches: broad}
    - has: {kind: as_pattern, has: {nthChild: 1, matches: broad}}
  not:
    has:
      kind: block
      has:
        stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}
        any:
          - {kind: raise_statement, regex: '^raise(\s+\w+)?\s*$'}
          - {kind: raise_statement, regex: '\sfrom\s+\w+\s*$'}
          - {kind: call, has: {field: function, kind: attribute, regex: '\.exception$'}}
          - {kind: keyword_argument, has: {field: name, regex: '^exc_info$'}, not: {has: {field: value, regex: '^(False|None|0)$'}}}
message: "Catch the exceptions you expect, or re-raise or log this one"
```

## How to fix

1. Catch the specific exceptions the block can raise; or, where a broad handler is meant, log with `logger.exception(...)` or re-raise.

## Examples

### Python

#### Don't

```python
try:
    sync()
except Exception:
    pass
```

#### Do

```python
try:
    sync()
except ConnectionError:
    pass
```
