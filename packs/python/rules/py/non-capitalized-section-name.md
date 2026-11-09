---
id: py/non-capitalized-section-name
title: "A docstring section name is capitalized"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/D405]
since: 0.5.0
---

The name of a docstring section is written as the convention spells it (`Args`, `Returns`, `See Also`), not in lower case or shouted. The fix rewrites the name. The check reads the docstring's sections as Ruff does with no convention set: NumPy style when there is a `Parameters` section, Google style when there is an `Args` section, otherwise whichever finds more sections.

**Why:** Tools that read the sections match the spelling; `args:` is a line of text to them.

```codelaw
native: py/non-capitalized-section-name
```

## Examples

### Python

#### Don't

```python
def f(a):
    """Return a.

    args:
        a: The value.

    """
```

#### Do

```python
def f(a):
    """Return a.

    Args:
        a: The value.

    """
```
