# django/django-non-leading-receiver-decorator — cases

### case: stacked receivers are fine

```python
@receiver(post_save)
@receiver(post_delete)
def f(sender, **kw):
    pass
```

```expect

```

### case: receiver first is right

```python
@receiver(post_save)
@atomic
def f(sender, **kw):
    pass
```

```expect

```
