---
id: py/quoted-type-alias
title: "A type alias that can run without quotes is not quoted"
languages: [python]
fix: auto
fix_reason: "auto from Python 3.10, where `X | Y` can run, and when no comment sits in the quoted value (a comment is lost: that rewrite is a suggestion); below the target, or with none set, the finding is a suggestion"
fix_target: { python: "3.10" }
stability: preview
ported_from: [ruff/TC008]
since: 0.5.0
---

A string in the value of a type alias is a forward reference that is not needed, and its quotes are removed. In an explicit alias (`X: TypeAlias = …`) the strings read are the ones joined by `|`, in a subscript, a list or a tuple — not inside `Literal[...]` and not after the first argument of `Annotated[...]` — and a string is kept when the value would not run without it: it names a module-level binding that does not exist yet at that point or that exists only under `if TYPE_CHECKING:` (unless the alias is itself under it), or it holds a subscript or an attribute outside a `TYPE_CHECKING` block (which may exist only in a stub). In a `type X = …` statement nothing is evaluated early, so every string in the value is reported. A multi-line string gets parentheses when it has none (`"""int\n| None"""` becomes `(int\n| None)`). The finding is reported for every target; a `X | Y` value is Python 3.10's, and the edit is automatic only when the project's target is. The rule does not know whether `py/runtime-string-union` is on: `"int" | None` is reported here.

**Why:** A quoted alias is a string that every reader and every tool has to parse again, and a runtime library that reads the alias sees a string where the type was meant.

```codelaw
native: py/quoted-type-alias
```

## Examples

### Python

#### Don't

```python
from typing import TypeAlias

MaybeInt: TypeAlias = "int | None"
```

#### Do

```python
from typing import TypeAlias

MaybeInt: TypeAlias = int | None
```
