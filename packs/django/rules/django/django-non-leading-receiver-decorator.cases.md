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

### case: two decorators are swapped, in a class too

```python
class C:
    @atomic
    @receiver(post_save)
    def f(sender, **kw):
        pass
```

```expect
3:5 django/django-non-leading-receiver-decorator
```

```fixed
class C:
    @receiver(post_save)
    @atomic
    def f(sender, **kw):
        pass
```

### case: three decorators carry no edit

```python
@a
@b
@receiver(post_save)
def f(sender, **kw):
    pass
```

```expect
3:1 django/django-non-leading-receiver-decorator
```
