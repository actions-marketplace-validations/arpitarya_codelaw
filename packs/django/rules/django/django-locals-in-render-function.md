---
id: django/django-locals-in-render-function
title: "No locals() as a template context"
languages: [python]
fix: manual
fix_reason: "the context's keys are the author's to list"
fix_pending: "suggest; needs scope-rename"
stability: preview
ported_from: [ruff/DJ003]
since: 0.5.0
---

`render()` is not passed `locals()` as its context.

**Why:** It sends every local variable to the template, hiding what the template uses and leaking more than it needs.

```codelaw
match:
  any:
    - pattern: render($R, $T, locals())
    - pattern: render($R, $T, context=locals())
message: "Pass an explicit context dict instead of `locals()`"
```

## How to fix

1. Pass a dict of the names the template uses.

## Examples

### Python

#### Don't

```python
def view(request):
    items = load()
    return render(request, 'list.html', locals())
```

#### Do

```python
def view(request):
    items = load()
    return render(request, 'list.html', {'items': items})
```
