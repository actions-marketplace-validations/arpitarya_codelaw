---
id: py/write-whole-file
title: "pathlib writes a whole file in one call"
languages: [python]
fix: suggest
fix_reason: "`Path(...).write_text(...)` opens, writes and closes the file as the block did, with the same default encoding; `Path` is imported from `pathlib` where the file does not"
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
    - has: {kind: with_clause, has: {kind: with_item, has: {kind: as_pattern, has: {kind: call, has: {field: arguments, has: {nthChild: 1, pattern: $P}}}}}}
    - any:
        - all: [{regex: '^with\s+open\([^\n]*["'']wb["'']'}, {pattern: $BYTES}]
        - all: [{not: {regex: '^with\s+open\([^\n]*["'']wb["'']'}}, {pattern: $TEXT}]
message: "Write it with `Path(...).write_text(...)`"
fix:
  - replace: { target: $TEXT, with: "Path($P).write_text($D)" }
  - replace: { target: $BYTES, with: "Path($P).write_bytes($D)" }
  - ensure-import: { module: pathlib, name: Path }
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
from pathlib import Path

Path(path).write_text(text)
```
