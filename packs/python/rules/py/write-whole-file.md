---
id: py/write-whole-file
title: "pathlib writes a whole file in one call"
languages: [python]
fix: manual
fix_reason: "the block becomes `Path(p).write_text(data)`, which needs `pathlib.Path` imported"
fix_pending: "suggest; needs add-import (W-162)"
stability: preview
ported_from: [ruff/FURB103]
since: 0.5.0
---

`with open(p, "w") as f:` whose body is only `f.write(data)` is written `Path(p).write_text(data)` (`write_bytes` for mode `"wb"`).

**Why:** One call opens, writes and closes the file, and says so.

```codelaw
match:
  kind: with_statement
  all:
    - has: {kind: with_clause, all: [{has: {kind: with_item, has: {kind: as_pattern, all: [{has: {kind: call, all: [{has: {field: function, regex: '^open$'}}], has: {field: arguments, all: [{has: {nthChild: 2, kind: string, regex: '^["'']wb?["'']$'}}, {not: {has: {nthChild: 3}}}]}}}, {has: {kind: as_pattern_target, has: {kind: identifier, pattern: $F}}}]}}}, {not: {has: {nthChild: 2}}}]}
    - has:
        field: body
        all:
          - has: {nthChild: 1, kind: expression_statement, has: {kind: call, pattern: $F.write($D)}}
          - not: {has: {nthChild: 2}}
message: "Write it with `Path(...).write_text(...)`"
```

## How to fix

1. Replace the block with `Path(p).write_text(data)` (or `write_bytes`), importing `pathlib.Path`.

## Examples

### Python

#### Don't

```python
with open(path, "w") as f:
    f.write(text)
```

#### Do

```python
Path(path).write_text(text)
```
