---
id: py/non-lowercase-variable-in-function
title: "Variables in a function are lowercase"
languages: [python]
fix: suggest
fix_reason: "the local and its uses in the function are renamed; code that reads it through `locals()` or `eval` would not follow"
fix_exception: value
stability: preview
ported_from: [ruff/N806, pylint/invalid-name]
since: 0.5.0
---

A variable assigned inside a function has no capital letter: `total`, not `Total` or `TOTAL`. An assignment of a `TypeVar`, `ParamSpec`, `TypeVarTuple`, `NewType`, `NamedTuple` or `TypedDict` call, which names a type, is not reported.

**Why:** PEP 8 names variables in lowercase; capitals in a function read as a class or a module constant.

```codelaw
utils:
  target:
    any:
      - inside: {kind: assignment, field: left}
      - inside: {kind: augmented_assignment, field: left}
      - inside: {any: [{kind: pattern_list}, {kind: tuple_pattern}], inside: {kind: assignment, field: left}}
      - inside: {any: [{kind: pattern_list}, {kind: tuple_pattern}], inside: {kind: for_statement, field: left}}
      - inside: {kind: for_statement, field: left}
match:
  kind: identifier
  pattern: $N
  regex: '[A-Z]'
  matches: target
  inside: {kind: function_definition, stopBy: end}
  all:
    - not:
        inside:
          kind: assignment
          has: {field: right, kind: call, has: {field: function, regex: '^(typing\.|typing_extensions\.)?(TypeVar|ParamSpec|TypeVarTuple|NewType|NamedTuple|TypedDict)$'}}
    - not: {inside: {kind: class_definition, stopBy: {kind: function_definition}}}
transform:
  U: { replace: { source: $N, replace: '^(_*).*$', by: '$1' } }
  S: { convert: { source: $N, toCase: snakeCase } }
message: "Name the variable in lowercase: `$U$S`"
fix:
  - rename-symbol-local: {target: $N, to: $U$S}
```

## Examples

### Python

#### Don't

```python
def area(w, h):
    Total = w * h
    return Total
```

#### Do

```python
def area(w, h):
    total = w * h
    return total
```
