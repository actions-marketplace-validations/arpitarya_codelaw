---
id: py/debugger
title: "No debugger calls"
languages: [python]
fix: suggest
fix_reason: "the call is removed; review that it was not meant to stay"
stability: preview
ported_from: [ruff/T100, pylint/forgotten-debug-statement]
since: 0.5.0
---

`breakpoint()`, `pdb.set_trace()`, `ipdb.set_trace()` and similar debugger calls are removed.

**Why:** Left in, they stop the program waiting for a debugger.

```codelaw
match:
  kind: expression_statement
  has: {kind: call, nthChild: 1, has: {field: function, regex: '^(breakpoint|(pdb|ipdb|pudb|pdbr|celery\.contrib\.rdb|debugpy|ptvsd)\.(set_trace|post_mortem|breakpoint|wait_for_client)|rdb\.set_trace)$'}}
  not: {inside: {kind: block, not: {has: {nthChild: 2}}}}
message: "Remove the debugger call"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
def f(x):
    breakpoint()
    return x
```

#### Do

```python
def f(x):
    return x
```
