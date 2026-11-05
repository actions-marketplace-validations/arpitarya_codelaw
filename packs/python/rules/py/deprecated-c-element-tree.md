---
id: py/deprecated-c-element-tree
title: "xml.etree.ElementTree, not cElementTree"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP023]
since: 0.5.0
---

`xml.etree.cElementTree` is imported as `xml.etree.ElementTree`.

**Why:** `cElementTree` was removed in Python 3.9.

```codelaw
match:
  kind: dotted_name
  regex: '^xml\.etree\.cElementTree$'
  inside: {any: [{kind: import_statement}, {kind: import_from_statement}, {kind: aliased_import, inside: {kind: import_statement}}]}
message: "Import `xml.etree.ElementTree`"
fix: "xml.etree.ElementTree"
```

## Examples

### Python

#### Don't

```python
import xml.etree.cElementTree as ET
```

#### Do

```python
import xml.etree.ElementTree as ET
```
