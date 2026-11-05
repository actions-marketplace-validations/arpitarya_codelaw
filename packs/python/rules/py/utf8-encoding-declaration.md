---
id: py/utf8-encoding-declaration
title: "No coding: utf-8 comment"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP009]
since: 0.5.0
---

A `# -*- coding: utf-8 -*-` declaration on one of the first two lines is removed.

**Why:** UTF-8 is Python 3's default source encoding.

```codelaw
match:
  kind: comment
  regex: '^#.*coding[:=]\s*(utf-?8|UTF-?8)\b'
  inside: {kind: module}
  not: {follows: {stopBy: end, not: {kind: comment}}}
message: "UTF-8 is the default; remove the declaration"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
# -*- coding: utf-8 -*-
import os
```

#### Do

```python
import os
```
