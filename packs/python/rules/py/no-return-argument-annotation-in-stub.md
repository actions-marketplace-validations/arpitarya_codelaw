---
id: py/no-return-argument-annotation-in-stub
title: "A stub's parameter is annotated Never, not NoReturn"
languages: [python]
files: ["**/*.pyi"]
fix: auto
fix_target: { python: "3.11" }
stability: preview
ported_from: [ruff/PYI050]
since: 0.5.0
---

In a stub, a parameter that can take no value is annotated `Never`, not `NoReturn`: `def stop(reason: Never) -> None: ...`. `NoReturn` and `typing.NoReturn` are reported on any parameter, `*args` and `**kwargs` included; a return annotation is not touched. Ruff also reports it in a `.py` file whose target is 3.11 or later; a finding cannot be gated on the target, so this rule reads stubs only.

**Why:** The two mean the same bottom type, but a parameter that "does not return" reads as nonsense; `Never` says what it is.

```codelaw
match:
  kind: type
  all:
    - any:
        - {inside: {kind: typed_parameter}}
        - {inside: {kind: typed_default_parameter}}
    - any:
        - all: [{has: {kind: identifier, regex: '^NoReturn$', pattern: $BARE}}, {not: {has: {nthChild: 2}}}]
        - all: [{has: {kind: attribute, regex: '^(typing|typing_extensions)\.NoReturn$', pattern: $QUAL}}, {not: {has: {nthChild: 2}}}]
message: "Annotate the parameter `Never`"
fix:
  - replace: { target: $BARE, with: "Never" }
  - edit-text: { target: $QUAL, pattern: 'NoReturn\z', replace: 'Never' }
  - ensure-import: { target: $BARE, module: typing, name: Never }
```

## How to fix

1. Write `Never` for `NoReturn` in the parameter's annotation, importing it from `typing` (Python 3.11+; `typing_extensions` before).

## Examples

### Python

#### Don't

```python path=api.pyi
from typing import NoReturn

def stop(reason: NoReturn) -> None: ...
```

#### Do

```python path=api.pyi
from typing import NoReturn, Never

def stop(reason: Never) -> None: ...
```
