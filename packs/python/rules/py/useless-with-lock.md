---
id: py/useless-with-lock
title: "No with on a fresh lock"
languages: [python]
fix: manual
fix_reason: "the lock must be created once and shared, which moves it out of the statement"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLW2101]
since: 0.5.0
---

A `with` statement does not acquire a lock created in the same statement (`with threading.Lock():`).

**Why:** Each run makes a new lock that nothing else holds, so it protects nothing.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^(threading\.)?(Lock|RLock|Semaphore|BoundedSemaphore|Condition)$'}
  inside: {kind: with_item}
message: "This lock is new each time; it protects nothing"
```

## How to fix

1. Create the lock once (at module or instance level) and use that lock in `with`.

## Examples

### Python

#### Don't

```python
def update():
    with threading.Lock():
        counter.add()
```

#### Do

```python
def update():
    with LOCK:
        counter.add()
```
