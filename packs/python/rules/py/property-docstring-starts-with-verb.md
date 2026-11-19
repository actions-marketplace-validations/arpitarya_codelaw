---
id: py/property-docstring-starts-with-verb
title: "A property's docstring does not start with a verb"
languages: [python]
fix: manual
fix_reason: "a property's docstring names the value, and the sentence is the author's to write"
fix_exception: authored
stability: preview
options:
  property_decorators: { type: "string[]", default: [], description: "decorators, beside `property`, `functools.cached_property`, `abc.abstractproperty` and `types.DynamicClassAttribute`, that make a function a property (a qualified name, `gi.repository.GObject.Property`)" }
  ignore_decorators: { type: "string[]", default: [], description: "decorators (qualified names) whose functions are not read" }
ported_from: [ruff/D421]
since: 0.5.0
---

The docstring of a property does not start with `return`, `returns`, `get`, `gets`, `yield`, `yields`, `fetch`, `fetches`, `retrieve` or `retrieves`. A property is a function decorated with `property`, `functools.cached_property`, `abc.abstractproperty`, `types.DynamicClassAttribute` or a decorator named in `property_decorators`.

**Why:** The Google Python style guide asks that a property's docstring read like an attribute's (`"""The Bigtable path."""`), not like a function's (`"""Returns the Bigtable path."""`), because the property is used as a value.

```codelaw
native: py/property-docstring-starts-with-verb
```

## How to fix

1. Name the value instead of describing the call: `Returns the bar.` becomes `The bar.`.

## Examples

### Python

#### Don't

```python
class Foo:
    @property
    def bar(self):
        """Returns the bar."""
        return self._bar
```

#### Do

```python
class Foo:
    @property
    def bar(self):
        """The bar."""
        return self._bar
```
