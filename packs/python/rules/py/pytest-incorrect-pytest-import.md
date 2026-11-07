---
id: py/pytest-incorrect-pytest-import
title: "pytest is imported as pytest"
languages: [python]
fix: manual
fix_reason: "every use of the imported names is rewritten as `pytest.<name>`"
fix_exception: restructure
stability: preview
ported_from: [ruff/PT013]
since: 0.5.0
---

pytest is imported with `import pytest`: not `import pytest as pt`, and not `from pytest import fixture`.

**Why:** One spelling, `pytest.fixture`, `pytest.raises`, keeps test files consistent and greppable.

```codelaw
match:
  any:
    - kind: aliased_import
      has: {field: name, regex: '^pytest$'}
      inside: {kind: import_statement}
    - kind: import_from_statement
      has: {field: module_name, regex: '^pytest$'}
message: "Write `import pytest`"
```

## How to fix

1. Replace the import with `import pytest` and write each use as `pytest.<name>`.

## Examples

### Python

#### Don't

```python
from pytest import fixture
```

#### Do

```python
import pytest
```
