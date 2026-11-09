# airflow/airflow-task-branch-as-short-circuit — cases

### case: the operator form

```python
from airflow.operators.python import BranchPythonOperator


def pick():
    if ready():
        return ["load"]
    return []


branch = BranchPythonOperator(task_id="branch", python_callable=pick)
```

```expect
10:10 airflow/airflow-task-branch-as-short-circuit
```

### case: two lists are a real branch

```python
from airflow.decorators import task


@task.branch
def pick():
    if ready():
        return ["load"]
    return ["skip"]
```

```expect
(none)
```
