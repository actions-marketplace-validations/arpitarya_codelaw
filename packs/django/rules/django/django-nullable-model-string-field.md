---
id: django/django-nullable-model-string-field
title: "No null=True on string fields"
languages: [python]
fix: manual
fix_reason: "removing `null=True` needs a migration and a default for the existing NULLs"
fix_exception: restructure
stability: preview
ported_from: [ruff/DJ001]
since: 0.5.0
---

A Django string field (`CharField`, `TextField`, `SlugField`, `EmailField`, …) does not set `null=True`.

**Why:** A string field then has two empty values, `NULL` and `''`; Django's convention is the empty string.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: '^(models\.)?(CharField|TextField|SlugField|EmailField|FilePathField|URLField)$'}
  - has:
      field: arguments
      has: {kind: keyword_argument, regex: ^null\s*=\s*True$}
  - not:
      has:
        field: arguments
        has: {kind: keyword_argument, regex: ^unique\s*=\s*True$}
message: Avoid `null=True` on a string field; use `blank=True` and the empty string
```

## How to fix

1. Remove `null=True` (keep `blank=True` if the field is optional) and migrate existing NULLs to `''`.

## Examples

### Python

#### Don't

```python
class Book(models.Model):
    title = models.CharField(max_length=80, null=True)
```

#### Do

```python
class Book(models.Model):
    title = models.CharField(max_length=80, blank=True)
```
