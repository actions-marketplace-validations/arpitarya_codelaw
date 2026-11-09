# airflow/airflow-task-implicit-multiple-outputs — cases

### case: the body returns a dict, the annotation is not a mapping

```python
from airflow.decorators import task


@task(retries=1)
def extract():
    return dict(a=1)
```

```expect
4:1 airflow/airflow-task-implicit-multiple-outputs
```

```fixed
from airflow.decorators import task


@task(multiple_outputs=False, retries=1)
def extract():
    return dict(a=1)
```

### case: stated already

```python
from airflow.decorators import task


@task(multiple_outputs=True)
def extract():
    return {"a": 1}
```

```expect
(none)
```

### case: a TypedDict of the file

```python
from typing import TypedDict

from airflow.sdk import task


class Row(TypedDict):
    a: int


@task.python
def extract() -> Row:
    return {"a": 1}
```

```expect
10:1 airflow/airflow-task-implicit-multiple-outputs
```

### case: not a dict

```python
from airflow.decorators import task


@task
def extract() -> int:
    return 1
```

```expect
(none)
```
