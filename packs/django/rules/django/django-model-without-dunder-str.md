---
id: django/django-model-without-dunder-str
title: "A model defines __str__"
languages: [python]
fix: manual
fix_reason: "what represents the object is the author's to choose"
stability: preview
ported_from: [ruff/DJ008]
since: 0.5.0
---

A concrete Django model defines `__str__`.

**Why:** Without it the admin, the shell and error messages show `Book object (1)`.

```codelaw
match:
  kind: class_definition
  all:
  - has: {field: superclasses, regex: '\b(models\.)?Model\b'}
  - not:
      has:
        field: body
        has:
          kind: function_definition
          has: {field: name, regex: ^__str__$}
  - not:
      has:
        field: body
        has:
          kind: decorated_definition
          has:
            kind: function_definition
            has: {field: name, regex: ^__str__$}
  - not:
      has:
        field: body
        has:
          kind: class_definition
          has: {field: name, regex: ^Meta$}
          regex: abstract\s*=\s*True
message: Define `__str__` on this model
```

## How to fix

1. Add `def __str__(self): return self.<name field>`.

## Examples

### Python

#### Don't

```python
class Book(models.Model):
    title = models.CharField(max_length=80)
```

#### Do

```python
class Book(models.Model):
    title = models.CharField(max_length=80)

    def __str__(self):
        return self.title
```
