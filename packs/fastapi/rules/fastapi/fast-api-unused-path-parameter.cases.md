# fastapi/fast-api-unused-path-parameter — cases

### case: a converter in the placeholder

```python
from fastapi import FastAPI

app = FastAPI()


@app.get("/files/{file_path:path}")
async def read(q: str):
    return q
```

```expect
6:18 fastapi/fast-api-unused-path-parameter
```

```fixed
from fastapi import FastAPI

app = FastAPI()


@app.get("/files/{file_path:path}")
async def read(q: str, file_path):
    return q
```

### case: a dependency takes the parameter

```python
from fastapi import Depends, FastAPI

app = FastAPI()


def common(item_id: int):
    return item_id


@app.get("/items/{item_id}")
async def read(c=Depends(common)):
    return c
```

```expect
(none)
```

### case: an alias

```python
from typing import Annotated

from fastapi import FastAPI, Path

app = FastAPI()


@app.get("/items/{item-id}")
async def read(item: Annotated[int, Path(alias="item-id")]):
    return item
```

```expect
(none)
```

### case: positional-only is reported without a fix

```python
from fastapi import FastAPI

app = FastAPI()


@app.get("/items/{item_id}")
async def read(item_id, /):
    return item_id
```

```expect
6:18 fastapi/fast-api-unused-path-parameter
```

### case: not a FastAPI app

```python
@other.get("/items/{item_id}")
async def read():
    return 1
```

```expect
(none)
```
