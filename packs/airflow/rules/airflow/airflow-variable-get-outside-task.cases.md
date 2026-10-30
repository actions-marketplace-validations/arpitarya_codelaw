# airflow/airflow-variable-get-outside-task — cases

### case: inside a task is fine

```python
from airflow.models import Variable

@task
def t():
    return Variable.get('x')
```

```expect

```
