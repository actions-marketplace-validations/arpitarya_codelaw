---
id: py/missing-copyright-notice
title: "A file carries a copyright notice"
languages: [python]
fix: manual
fix_reason: "the holder and the year are the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/CPY001]
since: 0.5.0
---

A non-empty Python file has a comment with a copyright notice — `Copyright 2026 …`, `Copyright (C) 2026 …` or `© 2026 …` — in its first lines.

**Why:** Some projects require every file to say who holds its copyright, for when files are copied out alone.

```codelaw
match:
  kind: module
  has: {nthChild: 1}
  not: {has: {kind: comment, stopBy: end, regex: '(?i)copyright\s+((\(c\)|©)\s+)?\d{4}'}}
message: "Add a copyright notice"
```

## How to fix

1. Add a comment at the top of the file: `# Copyright 2026 <holder>`.

## Examples

### Python

#### Don't

```python
import os
```

#### Do

```python
# Copyright 2026 Example Ltd
import os
```
