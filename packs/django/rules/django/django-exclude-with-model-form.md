---
id: django/django-exclude-with-model-form
title: "ModelForm lists fields, not exclude"
languages: [python]
fix: manual
fix_reason: "the fields to list are the author's"
stability: preview
ported_from: [ruff/DJ006]
since: 0.5.0
---

A `ModelForm`'s `Meta` names the fields it uses in `fields`, not the ones it leaves out in `exclude`.

**Why:** With `exclude`, a field added to the model later appears in the form — and is writable by users — without anyone deciding so.

```codelaw
match:
  kind: assignment
  has: { field: left, regex: '^exclude$' }
  inside:
    kind: expression_statement
    inside:
      kind: block
      inside:
        kind: class_definition
        has: { field: name, regex: '^Meta$' }
        inside: { kind: block, inside: { kind: class_definition, has: { field: superclasses, regex: 'ModelForm' } } }
message: "List the form's `fields` instead of using `exclude`"
```

## How to fix

1. Replace `exclude` with `fields = [...]` naming each field the form shows.

## Examples

### Python

#### Don't

```python
class BookForm(forms.ModelForm):
    class Meta:
        model = Book
        exclude = ['owner']
```

#### Do

```python
class BookForm(forms.ModelForm):
    class Meta:
        model = Book
        fields = ['title', 'author']
```
