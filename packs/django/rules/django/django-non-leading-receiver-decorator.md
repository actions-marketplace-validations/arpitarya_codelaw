---
id: django/django-non-leading-receiver-decorator
title: "@receiver is the first decorator"
languages: [python]
fix: manual
fix_reason: "reordering decorators changes which wrapper is connected"
fix_pending: "suggest"
stability: preview
ported_from: [ruff/DJ013]
since: 0.5.0
---

`@receiver` is the outermost (first) decorator of a signal handler.

**Why:** Django connects whatever `@receiver` wraps; below another decorator it connects the undecorated function and the other decorator never runs.

```codelaw
match:
  kind: decorator
  regex: '^@(\w+\.)*receiver\b'
  follows: { kind: decorator, stopBy: end, not: { regex: '^@(\w+\.)*receiver\b' } }
  inside: { kind: decorated_definition }
message: "Put `@receiver` first, above the other decorators"
```

## How to fix

1. Move `@receiver(...)` to the top of the decorator list.

## Examples

### Python

#### Don't

```python
@transaction.atomic
@receiver(post_save, sender=Book)
def on_save(sender, **kwargs):
    pass
```

#### Do

```python
@receiver(post_save, sender=Book)
@transaction.atomic
def on_save(sender, **kwargs):
    pass
```
