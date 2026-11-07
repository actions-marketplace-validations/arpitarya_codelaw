---
id: py/invalid-argument-name
title: "Parameter names are lowercase"
languages: [python]
fix: suggest
fix_reason: "a caller that passes the argument by keyword must change too, so the rename is reviewed"
fix_exception: value
stability: preview
ported_from: [ruff/N803, pylint/invalid-name]
since: 0.5.0
---

A function's parameter name has no capital letter: `user_id`, not `userId`. A method marked `@override` is not reported, since its parameters follow the method it overrides.

**Why:** PEP 8 names parameters in lowercase with underscores, as it names variables.

```codelaw
match:
  kind: identifier
  pattern: $N
  regex: '[A-Z]'
  any:
    - inside: {kind: parameters}
    - inside: {kind: default_parameter, field: name, inside: {kind: parameters}}
    - inside: {kind: typed_default_parameter, field: name, inside: {kind: parameters}}
    - inside: {kind: typed_parameter, inside: {kind: parameters}}
    - inside: {kind: list_splat_pattern, inside: {any: [{kind: parameters}, {kind: typed_parameter}]}}
    - inside: {kind: dictionary_splat_pattern, inside: {any: [{kind: parameters}, {kind: typed_parameter}]}}
  not:
    inside:
      kind: function_definition
      stopBy: end
      inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?override$'}}
transform:
  U: { replace: { source: $N, replace: '^(_*).*$', by: '$1' } }
  S: { convert: { source: $N, toCase: snakeCase } }
message: "Name the parameter in lowercase: `$U$S`"
fix:
  - rename-symbol-local: {target: $N, to: $U$S}
```

## Examples

### Python

#### Don't

```python
def load(userId, *, cacheTTL=60):
    return fetch(userId, cacheTTL)
```

#### Do

```python
def load(user_id, *, cache_ttl=60):
    return fetch(user_id, cache_ttl)
```
