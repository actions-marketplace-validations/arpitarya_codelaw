# airflow/airflow3-incompatible-function-signature — cases

### case: create_asset on a collector variable

```python
from airflow.lineage.hook import HookLineageCollector

collector = HookLineageCollector()
collector.create_asset("s3://bucket/key")
```

```expect
4:11 airflow/airflow3-incompatible-function-signature
```

### case: keywords are fine

```python
from airflow.lineage.hook import HookLineageCollector

collector = HookLineageCollector()
collector.create_asset(uri="s3://bucket/key")
```

```expect
(none)
```

### case: get_link with a bad signature

```python
from airflow.models import BaseOperatorLink


class Link(BaseOperatorLink):
    def get_link(self, operator, dttm):
        return ""
```

```expect
5:9 airflow/airflow3-incompatible-function-signature
```

### case: get_link with ti_key

```python
from airflow.sdk import BaseOperatorLink


class Link(BaseOperatorLink):
    def get_link(self, operator, *, ti_key):
        return ""
```

```expect
(none)
```
