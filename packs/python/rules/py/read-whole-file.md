---
id: py/read-whole-file
title: "pathlib reads a whole file in one call"
languages: [python]
fix: manual
fix_reason: "the block becomes `Path(p).read_text()`, which needs `pathlib.Path` imported"
fix_pending: "suggest; needs add-import (W-162)"
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
message: "Read it with `Path(...).read_text()`"
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
text = Path(path).read_text()
```
