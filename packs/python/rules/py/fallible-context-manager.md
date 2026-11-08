---
id: py/fallible-context-manager
title: "A `@contextmanager` runs its cleanup even when the body raises"
languages: [python]
fix: manual
fix_reason: "which statements are the cleanup, and what to catch, is the author's; the fix wraps the `yield` in `try`/`finally`"
fix_exception: restructure
stability: preview
ported_from: [ruff/RUF075]
since: 0.5.0
---

In a function decorated `@contextmanager` or `@asynccontextmanager` (from `contextlib`), a `yield` statement outside any `try` body is followed by more statements in its block. When the `with` body raises, the exception comes out of that `yield`, and the statements after it never run. Ruff also follows a `yield` that ends an `if` or a loop with code after the block; this rule reads the statements after the `yield` in its own block, and leaves alone a `return` or `break` straight after it, and a `yield` that ends a `with` block.

**Why:** The cleanup after the `yield` — closing a file, releasing a lock, restoring a setting — is skipped exactly when something went wrong.

```codelaw
match:
  kind: expression_statement
  has: {kind: yield}
  all:
    - inside:
        stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}
        kind: function_definition
        inside:
          kind: decorated_definition
          has: {kind: decorator, regex: '^@\s*(contextlib\.)?(async)?contextmanager\s*$'}
    - not:
        inside:
          stopBy: {any: [{kind: function_definition}, {kind: lambda}, {kind: class_definition}]}
          kind: block
          inside: {kind: try_statement}
          field: body
    - precedes:
        stopBy: neighbor
        not: {any: [{kind: return_statement}, {kind: break_statement}]}
        any:
          - kind: expression_statement
          - kind: if_statement
          - kind: for_statement
          - kind: while_statement
          - kind: with_statement
          - kind: try_statement
          - kind: delete_statement
          - kind: raise_statement
          - kind: assert_statement
          - kind: continue_statement
          - kind: pass_statement
          - kind: match_statement
          - kind: global_statement
          - kind: nonlocal_statement
message: "The code after this `yield` is skipped if the `with` body raises; use `try`/`finally`"
```

## How to fix

1. Wrap the `yield` in `try:` and move the cleanup into `finally:`.

## Examples

### Python

#### Don't

```python
from contextlib import contextmanager


@contextmanager
def opened(path):
    handle = open(path)
    yield handle
    handle.close()
```

#### Do

```python
from contextlib import contextmanager


@contextmanager
def opened(path):
    handle = open(path)
    try:
        yield handle
    finally:
        handle.close()
```
