---
id: py/non-imperative-mood
title: "A function's docstring starts in the imperative mood"
languages: [python]
fix: manual
fix_reason: "the right verb form is the author's: `Returns` becomes `Return`, but a first word that is a noun phrase needs the sentence rewritten"
fix_exception: authored
stability: preview
options:
  property_decorators: { type: "string[]", default: [], description: "decorators, beside `property`, `functools.cached_property`, `abc.abstractproperty` and `types.DynamicClassAttribute`, that make a function a property (a qualified name, `gi.repository.GObject.Property`); a property's docstring is not read" }
  ignore_decorators: { type: "string[]", default: [], description: "decorators (qualified names) whose functions are not read" }
ported_from: [ruff/D401]
since: 0.5.0
---

The first line of a function's docstring is written as a command: `Return the mean.`, not `Returns the mean.`. The first word is judged from pydocstyle's word lists, which `data/python/imperatives.json` holds: a word on its blacklist (`this`, `the`, `a`, `constructor`, `helper`) is not a verb; otherwise the word is reduced to its stem, and when the stem is one of the roughly 230 listed verbs, the word must be that verb's imperative form (`Returns`, `Returning` and `Returned` are not `Return`). A first word that belongs to no listed verb is not judged. A test function (a name starting `test`, or `runTest`) and a property are not read.

**Why:** [PEP 257](https://peps.python.org/pep-0257/) asks that the summary describe the function's effect as a command, so every docstring in a project reads the same.

```codelaw
native: py/non-imperative-mood
```

## How to fix

1. Rewrite the first word as the verb's imperative form: `Returns` to `Return`, `Gets` to `Get`.
2. For a first word that is a noun phrase (`The mean of the values.`), begin with the verb that says what the function does.

## Examples

### Python

#### Don't

```python
def average(values):
    """Returns the mean of the given values."""
    return sum(values) / len(values)
```

#### Do

```python
def average(values):
    """Return the mean of the given values."""
    return sum(values) / len(values)
```
