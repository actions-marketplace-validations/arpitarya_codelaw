---
id: py/import-outside-top-level
title: "Imports are at the top of the module"
languages: [python]
fix: manual
fix_reason: "the import moves to the module's top, or stays if it breaks a cycle or defers a slow module on purpose"
fix_exception: restructure
stability: preview
ported_from: [ruff/PLC0415, pylint/import-outside-toplevel]
since: 0.5.0
---

An `import` is not inside a function or method.

**Why:** An import at the top shows a module's dependencies in one place, and fails at import time rather than at the first call.

```codelaw
match:
  any: [{kind: import_statement}, {kind: import_from_statement}]
  inside: {kind: function_definition, stopBy: end}
message: "Move the import to the top of the module"
```

## How to fix

1. Move the import to the top of the module; if it is there on purpose (an import cycle, a slow optional module), suppress the finding with a reason.

## Examples

### Python

#### Don't

```python
def load():
    import json
    return json.loads(data)
```

#### Do

```python
import json


def load():
    return json.loads(data)
```
