---
id: py/global-variable-undefined
title: "A `global` name is defined at module level"
languages: [python]
fix: manual
fix_reason: "the module-level starting value is the author's to choose"
fix_exception: value
stability: preview
ported_from: [pylint/global-variable-undefined]
since: 0.5.0
---

A function declares `global NAME` and assigns it, and the module never binds `NAME` at its top level — by an assignment, a `def`, a `class` or an import. The variable then exists only once the function has run. A `global` the function never assigns after it is a different check, and is not reported here. A `global` that names several variables is read for its first.

**Why:** Code that reads the variable before the function runs gets a `NameError`, and a reader of the module cannot see that it has the variable at all.

```codelaw
match:
  kind: global_statement
  has: {kind: identifier, pattern: $N}
  all:
    - inside: {stopBy: end, kind: function_definition}
    - precedes:
        stopBy: end
        has:
          stopBy: end
          any:
            - {kind: assignment, has: {field: left, kind: identifier, pattern: $N}}
            - {kind: augmented_assignment, has: {field: left, kind: identifier, pattern: $N}}
            - {kind: pattern_list, has: {kind: identifier, pattern: $N}}
            - {kind: for_statement, has: {field: left, kind: identifier, pattern: $N}}
    - not:
        inside:
          stopBy: end
          kind: module
          has:
            any:
              - {kind: expression_statement, has: {kind: assignment, has: {field: left, any: [{kind: identifier, pattern: $N}, {kind: pattern_list, has: {kind: identifier, pattern: $N}}]}}}
              - {kind: function_definition, has: {field: name, pattern: $N}}
              - {kind: class_definition, has: {field: name, pattern: $N}}
              - {kind: decorated_definition, has: {field: definition, has: {field: name, pattern: $N}}}
              - {kind: import_statement, has: {stopBy: end, kind: identifier, pattern: $N}}
              - {kind: import_from_statement, has: {stopBy: end, kind: identifier, pattern: $N}}
              - {kind: if_statement, has: {stopBy: end, kind: assignment, has: {field: left, kind: identifier, pattern: $N}}}
              - {kind: try_statement, has: {stopBy: end, any: [{kind: assignment, has: {field: left, kind: identifier, pattern: $N}}, {kind: import_statement, has: {stopBy: end, kind: identifier, pattern: $N}}, {kind: import_from_statement, has: {stopBy: end, kind: identifier, pattern: $N}}]}}
message: "`$N` is declared `global` but the module never defines it"
```

## How to fix

1. Give the variable a value at module level, next to the module's other state.

## Examples

### Python

#### Don't

```python
def connect():
    global client
    client = make_client()
```

#### Do

```python
client = None


def connect():
    global client
    client = make_client()
```
