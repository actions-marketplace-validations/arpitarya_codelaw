### case: two fields keep their order

```python
class Book(models.Model):
    def __str__(self):
        return self.title

    title = models.CharField(max_length=80)

    def save(self):
        pass

    pages = models.IntegerField()
```

```expect
5:5 django/django-unordered-body-content-in-model
10:5 django/django-unordered-body-content-in-model
```

```fixed
class Book(models.Model):
    title = models.CharField(max_length=80)

    pages = models.IntegerField()

    def __str__(self):
        return self.title

    def save(self):
        pass
```
