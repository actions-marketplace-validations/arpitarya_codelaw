# django/django-model-without-dunder-str — cases

### case: an abstract model needs none

```python
class Base(models.Model):
    class Meta:
        abstract = True
```

```expect

```
