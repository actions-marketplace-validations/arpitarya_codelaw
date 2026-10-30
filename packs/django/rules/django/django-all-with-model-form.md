---
id: django/django-all-with-model-form
title: "ModelForm does not use fields = '__all__'"
languages: [python]
fix: manual
fix_reason: "the fields to list are the author's"
stability: preview
ported_from: [ruff/DJ007]
since: 0.5.0
---

A `ModelForm`'s `Meta` does not set `fields = "__all__"`.

**Why:** Every model field, including ones added later, becomes user-editable without anyone deciding so.

```codelaw
match:
  kind: assignment
  all:
  - has: {field: left, regex: ^fields$}
  - has: {field: right, regex: '^["'']__all__["'']$'}
  - inside:
      kind: expression_statement
      inside:
        kind: block
        inside:
          kind: class_definition
          has: {field: name, regex: ^Meta$}
          inside:
            kind: block
            inside:
              kind: class_definition
              has: {field: superclasses, regex: ModelForm}
message: List the form's fields instead of `"__all__"`
```

## How to fix

1. Set `fields` to the list of fields the form shows.

## Examples

### Python

#### Don't

```python
class BookForm(forms.ModelForm):
    class Meta:
        model = Book
        fields = '__all__'
```

#### Do

```python
class BookForm(forms.ModelForm):
    class Meta:
        model = Book
        fields = ['title', 'author']
```
