---
id: py/undefined-future-feature
title: "`from __future__` names only real features"
languages: [python]
fix: suggest
fix_reason: "the features within two edits of the name are choices, the nearest the default, beside removing the name; a SyntaxError today, so each choice changes what runs"
stability: preview
ported_from: [ruff/F407]
since: 0.1.0
---

Every name imported from `__future__` is a feature Python defines.

**Why:** An unknown feature is a `SyntaxError` when the module is compiled.

```codelaw
native: py/undefined-future-feature
```

## How to fix

1. Check the name against the `__future__` module's features.
2. Correct it or remove the import.

## Examples

### Python

#### Don't

```py
from __future__ import annotatons
```

#### Fixed (annotations)

```py
from __future__ import annotations
```

#### Fixed (remove)

```py
```

#### Do

```py
from __future__ import annotations
```
