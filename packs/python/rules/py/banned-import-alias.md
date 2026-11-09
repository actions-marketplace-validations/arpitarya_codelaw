---
id: py/banned-import-alias
title: "A module is not imported under a banned alias"
languages: [python]
fix: manual
fix_reason: "which alias to use instead is the project's convention"
fix_exception: value
stability: preview
options: { banned_aliases: { type: "object", default: {}, description: "module → the aliases it must not be imported as" } }
ported_from: [ruff/ICN002]
since: 0.5.0
---

`import module as alias` and `from package import name as alias` do not use an alias that `banned_aliases` lists for that module (`package.name` for the second form). Nothing is banned by default.

**Why:** A project that fixes the alias of a module (`np` for NumPy) keeps readers from learning several.

```codelaw
native: py/banned-import-alias
```

## How to fix

1. Import the module under the project's alias, or without one.

## Examples

### Python

#### Don't

```python options={"banned_aliases":{"numpy":["nmp"]}}
import numpy as nmp
```

#### Do

```python options={"banned_aliases":{"numpy":["nmp"]}}
import numpy as np
```
