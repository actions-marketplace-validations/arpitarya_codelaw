---
id: py/attribute-defined-outside-init
title: "Instance attributes are first set in `__init__`"
languages: [python]
fix: manual
fix_reason: "the attribute's starting value in `__init__` is the author's to choose"
fix_exception: value
stability: preview
ported_from: [pylint/attribute-defined-outside-init]
since: 0.5.0
---

A method assigns `self.<name>` that the class never sets in `__init__`, `__new__`, `setUp`, `asyncSetUp` or `__post_init__`, and does not declare in its body. Pylint follows the class hierarchy and the methods `__init__` calls; this rule reads one class on its own, so it checks only a class with no base classes (or only `object`), whose name does not end in `Mixin`, and leaves alone an assignment in a method that a defining method calls (`self._reset()` from `__init__`) and in a property setter.

**Why:** An attribute that appears only once some method has run makes the object's shape depend on call order, and an `AttributeError` waits for the code path that reads it first.

```codelaw
match:
  kind: assignment
  all:
    - has:
        field: left
        kind: attribute
        pattern: self.$X
    - inside:
        stopBy: end
        kind: function_definition
        all:
          - has: {field: name, pattern: $M}
          - not: {has: {field: name, regex: '^(__init__|__new__|setUp|asyncSetUp|__post_init__)$'}}
        inside:
          kind: block
          inside:
            kind: class_definition
            all:
              - not: {has: {field: name, regex: 'Mixin$'}}
              - not:
                  has:
                    field: superclasses
                    kind: argument_list
                    has:
                      any: [{kind: identifier}, {kind: attribute}, {kind: subscript}, {kind: call}, {kind: keyword_argument}]
                      not: {regex: '^object$'}
              - not:
                  has:
                    field: body
                    has:
                      stopBy: end
                      kind: function_definition
                      all:
                        - has: {field: name, regex: '^(__init__|__new__|setUp|asyncSetUp|__post_init__)$'}
                        - has:
                            stopBy: end
                            any:
                              - {kind: attribute, pattern: self.$X}
                              - {kind: call, pattern: self.$M($$$)}
              - not:
                  has:
                    field: body
                    has:
                      kind: expression_statement
                      has: {kind: assignment, has: {field: left, kind: identifier, pattern: $X}}
    - not:
        inside:
          stopBy: end
          kind: decorated_definition
          has: {kind: decorator, regex: '\.(setter|deleter)$'}
message: "`self.$X` is first set outside `__init__`"
```

## How to fix

1. Give the attribute its starting value in `__init__` (often `None`).

## Examples

### Python

#### Don't

```python
class Session:
    def __init__(self):
        self.user = None

    def connect(self):
        self.socket = open_socket()
```

#### Do

```python
class Session:
    def __init__(self):
        self.user = None
        self.socket = None

    def connect(self):
        self.socket = open_socket()
```
