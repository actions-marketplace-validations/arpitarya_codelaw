---
id: py/read-whole-file
title: "pathlib reads a whole file in one call"
languages: [python]
fix: suggest
fix_reason: "`Path(...).read_text()` opens, reads and closes the file as the block did, with the same default encoding; `Path` is imported from `pathlib` where the file does not"
stability: preview
ported_from: [ruff/FURB101]
since: 0.5.0
---

`with open(p) as f:` whose body is only `x = f.read()` is written `x = Path(p).read_text()` (`read_bytes()` for mode `"rb"`).

**Why:** One call opens, reads and closes the file, and says so.

```codelaw
match:
  kind: with_statement
  all:
    - has: {kind: with_clause, all: [{has: {kind: with_item, has: {kind: as_pattern, all: [{has: {kind: call, all: [{has: {field: function, regex: '^open$'}}], has: {field: arguments, all: [{has: {nthChild: 1, not: {kind: keyword_argument}}}, {not: {has: {nthChild: 2, not: {kind: string, regex: '^["'']rb?["'']$'}}}}, {not: {has: {nthChild: 3}}}]}}}, {has: {kind: as_pattern_target, has: {kind: identifier, pattern: $F}}}]}}}, {not: {has: {nthChild: 2}}}]}
    - has:
        field: body
        all:
          - has: {nthChild: 1, kind: expression_statement, has: {kind: assignment, has: {field: right, pattern: $F.read()}}}
          - not: {has: {nthChild: 2}}
    - has: {kind: with_clause, has: {kind: with_item, has: {kind: as_pattern, has: {kind: call, has: {field: arguments, has: {nthChild: 1, pattern: $P}}}}}}
    - has: {field: body, has: {nthChild: 1, has: {kind: assignment, has: {field: left, pattern: $V}}}}
    - any:
        - all: [{regex: '^with\s+open\([^\n]*["'']rb["'']'}, {pattern: $BYTES}]
        - all: [{not: {regex: '^with\s+open\([^\n]*["'']rb["'']'}}, {pattern: $TEXT}]
message: "Read it with `Path(...).read_text()`"
fix:
  - replace: { target: $TEXT, with: "$V = Path($P).read_text()" }
  - replace: { target: $BYTES, with: "$V = Path($P).read_bytes()" }
  - ensure-import: { module: pathlib, name: Path }
```

## How to fix

1. Replace the block with `x = Path(p).read_text()` (or `read_bytes()`), importing `pathlib.Path`.

## Examples

### Python

#### Don't

```python
with open(path) as f:
    text = f.read()
```

#### Do

```python
from pathlib import Path

text = Path(path).read_text()
```
