---
id: py/unnecessary-dunder-call
title: "A dunder method is reached through its builtin or operator"
languages: [python]
fix: suggest
fix_reason: "the builtin or operator also tries the reflected method and checks the result, so the behaviour can differ for a class that relies on calling the method directly"
stability: preview
ported_from: [ruff/PLC2801]
since: 0.5.0
---

Code calls a dunder method directly where a builtin or an operator does the same: `items.__len__()` is `len(items)`, `a.__add__(b)` is `a + b`, `s.__contains__(x)` is `x in s`. The fix writes the builtin, or the operator where both operands are simple (a name, an attribute, a call, a subscript or a literal) and the call stands alone in its statement, an assignment, a `return` or an argument. Inside a method that is itself a dunder (`def __eq__(self, other): return self.x.__eq__(other.x)`), on `super()`, and with keyword arguments the call is left alone; the dunders without a builtin or binary operator (`__init__`, `__getitem__`, `__enter__`, `__pow__`, `__or__`, the reflected and in-place ones) are not read.

**Why:** The builtin and the operator are what readers expect, and they follow the full protocol — reflected methods, `NotImplemented`, result checks — that a direct call skips.

```codelaw
utils:
  simple: {any: [{kind: identifier}, {kind: attribute}, {kind: call}, {kind: subscript}, {kind: integer}, {kind: float}, {kind: string}, {kind: list}, {kind: tuple}, {kind: dictionary}, {kind: parenthesized_expression}, {kind: "true"}, {kind: "false"}, {kind: none}]}
  safe-place: {any: [{inside: {kind: expression_statement}}, {inside: {kind: assignment, field: right}}, {inside: {kind: return_statement}}, {inside: {kind: argument_list}}, {inside: {kind: keyword_argument, field: value}}]}
match:
  kind: call
  all:
    - not: {inside: {stopBy: end, kind: function_definition, has: {field: name, regex: '^__\w+__$'}}}
    - not: {has: {field: function, kind: attribute, has: {field: object, regex: '^super\s*\('}}}
    - not: {has: {field: arguments, has: {any: [{kind: keyword_argument}, {kind: list_splat}, {kind: dictionary_splat}]}}}
    - any:
        - all:
            - has: {field: function, kind: attribute, has: {field: attribute, regex: '^(__len__|__str__|__repr__|__bool__|__bytes__|__complex__|__dir__|__float__|__hash__|__int__|__iter__|__next__|__reversed__|__abs__|__round__)$'}}
            - pattern: $X.$B()
            - pattern: $CALL
        - all:
            - has: {field: function, kind: attribute, has: {field: attribute, regex: '^(__add__|__sub__|__mul__|__truediv__|__floordiv__|__mod__|__matmul__|__and__|__xor__|__lshift__|__rshift__|__eq__|__ne__|__lt__|__le__|__gt__|__ge__)$'}}
            - pattern: $X.$O($A)
            - has: {field: function, has: {field: object, matches: simple}}
            - has: {field: arguments, has: {matches: simple}}
            - matches: safe-place
            - pattern: $OPCALL
        - all:
            - has: {field: function, kind: attribute, has: {field: attribute, regex: '^__contains__$'}}
            - pattern: $X.__contains__($A)
            - has: {field: function, has: {field: object, matches: simple}}
            - has: {field: arguments, has: {matches: simple}}
            - matches: safe-place
            - pattern: $INCALL
        - all:
            - has: {field: function, kind: attribute, has: {field: attribute, any: [{regex: '^(__add__|__sub__|__mul__|__truediv__|__floordiv__|__mod__|__matmul__|__and__|__xor__|__lshift__|__rshift__|__eq__|__ne__|__lt__|__le__|__gt__|__ge__)$'}, {regex: '^__contains__$'}]}}
            - has: {field: arguments, has: {nthChild: 1}, not: {has: {nthChild: 2}}}
message: "Use the builtin or operator instead of calling the dunder method"
fix:
  - replace: { target: $CALL, with: "$B|map(__len__:len, __str__:str, __repr__:repr, __bool__:bool, __bytes__:bytes, __complex__:complex, __dir__:dir, __float__:float, __hash__:hash, __int__:int, __iter__:iter, __next__:next, __reversed__:reversed, __abs__:abs, __round__:round)($X)" }
  - replace: { target: $OPCALL, with: "$X $O|map(__add__:+, __sub__:-, __mul__:*, __truediv__:/, __floordiv__://, __mod__:%, __matmul__:@, __and__:&, __xor__:^, __lshift__:<<, __rshift__:>>, __eq__:==, __ne__:!=, __lt__:<, __le__:<=, __gt__:>, __ge__:>=) $A" }
  - replace: { target: $INCALL, with: "$A in $X" }
```

## Examples

### Python

#### Don't

```python
size = items.__len__()
```

#### Do

```python
size = len(items)
```
