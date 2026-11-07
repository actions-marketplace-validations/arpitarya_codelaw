---
id: py/missing-timeout
title: "An HTTP request has a timeout"
languages: [python]
fix: manual
fix_reason: "how long to wait is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/missing-timeout]
since: 0.5.0
---

A `requests.get`, `post`, `put`, `patch`, `delete`, `head`, `options` or `request` call passes `timeout=`.

**Why:** Without one, `requests` waits forever on a server that does not answer.

```codelaw
match:
  kind: call
  has: {field: function, regex: '^requests\.(get|post|put|patch|delete|head|options|request)$'}
  all:
    - not: {has: {field: arguments, has: {kind: keyword_argument, has: {field: name, regex: '^timeout$'}}}}
    - not: {has: {field: arguments, has: {kind: dictionary_splat}}}
message: "Pass `timeout=`"
```

## How to fix

1. Pass `timeout=` with the seconds the call may take.

## Examples

### Python

#### Don't

```python
r = requests.get(url)
```

#### Do

```python
r = requests.get(url, timeout=10)
```
