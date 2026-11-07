---
id: django/django-unordered-body-content-in-model
title: "A model's body follows Django's order"
languages: [python]
fix: suggest
fix_reason: "the field moves above the methods before it, one method a pass, keeping the fields' own order; a comment above the field stays where it was"
stability: preview
ported_from: [ruff/DJ012]
since: 0.5.0
---

Inside a model, fields come before `class Meta`, and both before the methods.

**Why:** Django's style guide orders a model as fields, custom managers, `Meta`, `__str__`, `save`, `get_absolute_url`, then other methods, so every model reads the same way.

```codelaw
match:
  any:
    - kind: expression_statement
      has: { kind: assignment, has: { field: right, regex: '^(models\.)?\w+Field\(' } }
    - kind: class_definition
      has: { field: name, regex: '^Meta$' }
  follows:
    stopBy: end
    any: [{kind: function_definition}, {kind: decorated_definition}]
    pattern: $M
  inside:
    kind: block
    inside: { kind: class_definition, has: { field: superclasses, regex: '\b(models\.)?Model\b' } }
message: "Put fields and `class Meta` before the model's methods"
fix:
  - move: { before: $M }
```

## How to fix

1. Move the fields to the top of the class and `class Meta` after them, before any method.

## Examples

### Python

#### Don't

```python
class Book(models.Model):
    def __str__(self):
        return self.title

    title = models.CharField(max_length=80)
```

#### Do

```python
class Book(models.Model):
    title = models.CharField(max_length=80)

    def __str__(self):
        return self.title
```

## Differences from the source

Fields and `Meta` after a method are reported; the source also orders managers, `__str__`, `save` and `get_absolute_url` among themselves.
