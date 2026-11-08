---
id: py/generator-return-from-iter-method
title: "A simple __iter__ returns Iterator, not Generator"
languages: [python]
fix: suggest
fix_reason: "`Iterator[T]` is what `iter()` promises and all a caller may use; a body that does more than yield, return or pass might rely on `send()`, so the change is reviewed"
stability: preview
ported_from: [ruff/PYI058]
since: 0.5.0
---

A method `__iter__(self)` annotated `Generator`, or `Generator[T, None, None]` (each of the last two `None` or `Any`), is annotated `Iterator[T]`; `__aiter__(self)` annotated `AsyncGenerator[T, None]` is annotated `AsyncIterator[T]`. The fix imports the iterator type from `collections.abc`. The method takes only `self`, is not `async`, and its body holds only `yield`s, a docstring, `...`, `pass` or a bare `return` (a stub's `...` included); `Generator` is read as imported by name from `typing`, `typing_extensions` or `collections.abc`, not written with its module.

**Why:** A caller of `iter()` gets an iterator and can only call `next()` on it; `Generator` promises `send()` and `throw()` too, which the class does not mean to offer.

```codelaw
match:
  kind: function_definition
  all:
    - not: {regex: '^async\b'}
    - inside: {kind: block, inside: {kind: class_definition}}
    - has: {field: parameters, all: [{has: {nthChild: 1, kind: identifier}}, {not: {has: {nthChild: 2}}}]}
    - not:
        has:
          field: body
          has:
            not:
              any:
                - kind: pass_statement
                - {kind: return_statement, not: {has: {nthChild: 1, not: {kind: none}}}}
                - {kind: expression_statement, has: {nthChild: 1, any: [{kind: yield}, {kind: string}, {kind: ellipsis}]}}
    - inside: {kind: module, stopBy: end, has: {kind: import_from_statement, regex: '^from\s+(typing|typing_extensions|collections\.abc)\s+import\b[^\n]*\b(Async)?Generator\b'}}
    - any:
        - all:
            - has: {field: name, regex: '^__iter__$'}
            - any:
                - has: {field: return_type, has: {nthChild: 1, kind: identifier, regex: '^Generator$', pattern: $G}}
                - has:
                    field: return_type
                    has:
                      kind: generic_type
                      pattern: $GS
                      all:
                        - has: {nthChild: 1, kind: identifier, regex: '^Generator$'}
                        - has:
                            kind: type_parameter
                            all:
                              - has: {nthChild: 1, kind: type, pattern: $Y}
                              - has: {nthChild: 2, kind: type, regex: '^(None|Any)$'}
                              - has: {nthChild: 3, kind: type, regex: '^(None|Any)$'}
                              - not: {has: {nthChild: 4}}
        - all:
            - has: {field: name, regex: '^__aiter__$'}
            - any:
                - has: {field: return_type, has: {nthChild: 1, kind: identifier, regex: '^AsyncGenerator$', pattern: $A}}
                - has:
                    field: return_type
                    has:
                      kind: generic_type
                      pattern: $AS
                      all:
                        - has: {nthChild: 1, kind: identifier, regex: '^AsyncGenerator$'}
                        - has:
                            kind: type_parameter
                            all:
                              - has: {nthChild: 1, kind: type, pattern: $AY}
                              - has: {nthChild: 2, kind: type, regex: '^(None|Any)$'}
                              - not: {has: {nthChild: 3}}
message: "Annotate a simple `__iter__` or `__aiter__` as returning an iterator"
fix:
  - replace: { target: $G, with: "Iterator" }
  - replace: { target: $GS, with: "Iterator[$Y]" }
  - replace: { target: $A, with: "AsyncIterator" }
  - replace: { target: $AS, with: "AsyncIterator[$AY]" }
  - ensure-import: { target: $G, module: collections.abc, name: Iterator }
  - ensure-import: { target: $GS, module: collections.abc, name: Iterator }
  - ensure-import: { target: $A, module: collections.abc, name: AsyncIterator }
  - ensure-import: { target: $AS, module: collections.abc, name: AsyncIterator }
```

## How to fix

1. Annotate the method `Iterator[T]` (`AsyncIterator[T]` for `__aiter__`), imported from `collections.abc`.

## Examples

### Python

#### Don't

```python
from collections.abc import Generator


class Rows:
    def __iter__(self) -> Generator[int, None, None]:
        yield 1
```

#### Do

```python
from collections.abc import Generator, Iterator


class Rows:
    def __iter__(self) -> Iterator[int]:
        yield 1
```
