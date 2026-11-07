---
id: py/no-self-use
title: "A method that never reads `self` is a function or a static method"
languages: [python]
fix: manual
fix_reason: "making it a static method or a module function changes how callers reach it, which the author decides"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLR6301, pylint/no-self-use]
since: 0.5.0
---

A method in a class takes `self` and never uses it, nested functions and comprehensions included. Left alone, as upstream: dunder methods; a body that is only a docstring, `pass`, `...`, or `raise NotImplementedError` (with a message in a variable or not); a method decorated `@staticmethod`, `@classmethod`, `@abstractmethod`, `@override`, `@overload`, `@property`, `@cached_property` or a property's `.setter`/`.getter`/`.deleter`, or an attrs-style `@<field>.validator`; and a method that calls `super()`. This is a Pylint optional extension's check; it is in no preset.

**Why:** A method that does not depend on the instance hides that it is a plain function, and invites calls on objects it does not need.

```codelaw
match:
  kind: function_definition
  all:
    - inside: {kind: block, inside: {kind: class_definition}}
    - not: {has: {field: name, regex: '^__\w+__$'}}
    - has:
        field: parameters
        has: {nthChild: 1, kind: identifier, regex: '^self$', pattern: $S}
    - not:
        has:
          field: body
          not:
            has:
              not:
                any:
                  - kind: pass_statement
                  - {kind: expression_statement, regex: '^(\.\.\.|[rRuUbB]*("""|''''''|"|'')[\s\S]*)$'}
                  - {kind: raise_statement, regex: '^raise\s+(builtins\.)?(NotImplementedError|NotImplemented)\b'}
                  - {kind: expression_statement, has: {kind: assignment, has: {field: right, any: [{kind: string}, {kind: concatenated_string}]}}}
    - not: {has: {field: body, has: {stopBy: end, kind: identifier, regex: '^super$'}}}
    - not:
        inside:
          kind: decorated_definition
          has:
            kind: decorator
            regex: '^@\s*(((builtins|abc|typing|typing_extensions|functools)\.)?(staticmethod|classmethod|abstractmethod|abstractclassmethod|abstractstaticmethod|abstractproperty|override|overload|property|cached_property)|[\w.]+\.(setter|getter|deleter|validator))\b'
where:
  $S: unused
message: "This method never uses `self`"
```

## How to fix

1. Make it a `@staticmethod` and drop `self`, or move it out of the class as a function.
2. If subclasses override it and use `self`, mark it `@typing.override` in them, or leave it and say why.

## Examples

### Python

#### Don't

```python
class Report:
    def title(self):
        return "Monthly"
```

#### Do

```python
class Report:
    @staticmethod
    def title():
        return "Monthly"
```
