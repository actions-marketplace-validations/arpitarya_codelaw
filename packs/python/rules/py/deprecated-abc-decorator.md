---
id: py/deprecated-abc-decorator
title: "@abstractmethod under the plain decorator, not abc.abstractproperty"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP051]
since: 0.5.0
---

`abc.abstractclassmethod`, `abc.abstractstaticmethod` and `abc.abstractproperty`, deprecated since Python 3.3, are written as the plain decorator with `abstractmethod` below it: `@abc.abstractproperty` becomes `@property` over `@abc.abstractmethod`. A name imported from `abc` (`from abc import abstractproperty`) becomes `@abstractmethod`, imported from `abc` where the file does not import it.

**Why:** The three combined decorators are deprecated and may be removed; stacking the two decorators does the same and has worked since 3.3.

```codelaw
match:
  kind: decorator
  any:
    - all:
        - has: {kind: attribute, regex: '^abc\.abstract(classmethod|staticmethod|property)$', pattern: $Q, has: {field: attribute, pattern: $QN}}
        - pattern: $DQ
    - all:
        - has: {kind: identifier, regex: '^abstract(classmethod|staticmethod|property)$', pattern: $B}
        - pattern: $DB
        - inside: {kind: module, stopBy: end, has: {kind: import_from_statement, regex: '^from\s+abc\s+import\b[^\n]*\babstract(classmethod|staticmethod|property)\b'}}
transform:
  QTO: { substring: { source: $QN, startChar: 8 } }
  BTO: { substring: { source: $B, startChar: 8 } }
message: "Use `@property`, `@classmethod` or `@staticmethod` over `@abstractmethod`"
fix:
  - insert-before: { target: $DQ, text: "@$QTO", line: true }
  - replace: { target: $Q, with: "abc.abstractmethod" }
  - insert-before: { target: $DB, text: "@$BTO", line: true }
  - replace: { target: $B, with: "abstractmethod" }
  - ensure-import: { target: $B, module: abc, name: abstractmethod }
```

## How to fix

1. Write the plain decorator (`@property`, `@classmethod`, `@staticmethod`) above `@abstractmethod`.

## Examples

### Python

#### Don't

```python
import abc


class Shape(abc.ABC):
    @abc.abstractproperty
    def area(self): ...
```

#### Do

```python
import abc


class Shape(abc.ABC):
    @property
    @abc.abstractmethod
    def area(self): ...
```
