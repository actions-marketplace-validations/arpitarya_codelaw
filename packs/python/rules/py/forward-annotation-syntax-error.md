---
id: py/forward-annotation-syntax-error
title: "A string annotation is valid Python"
languages: [python]
fix: suggest
fix_reason: "when closing the brackets left open makes the string parse, they are added at its end, and the annotation starts to resolve to a type (`get_type_hints` raised before); any other broken annotation is manual, as the intended type is a value the code doesn't hold"
fix_exception: value
stability: preview
ported_from: [ruff/F722]
since: 0.1.0
---

A type annotation written as a string parses as a Python expression.

**Why:** Type checkers and `typing.get_type_hints()` fail on an annotation they can't parse.

```codelaw
native: py/forward-annotation-syntax-error
```

## How to fix

1. Correct the annotation so it parses.

## Examples

### Python

#### Don't

```py
def total(values: "list[int") -> int:
    return sum(values)
```

#### Do

```py
def total(values: "list[int]") -> int:
    return sum(values)
```

## Differences from the source

Strings inside `Literal[...]` and `Annotated[...]` are not treated as annotations.
