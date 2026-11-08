# py/blocking-http-call-httpx-in-async-function — cases

### case: a client built in the function

```py
import httpx


async def fetch(url):
    client = httpx.Client()
    return client.get(url)
```

```expect
6:12 py/blocking-http-call-httpx-in-async-function
```

### case: a client parameter annotated httpx.Client

```py
import httpx


async def fetch(client: httpx.Client, url):
    return client.post(url)
```

```expect
5:12 py/blocking-http-call-httpx-in-async-function
```

### case: a with statement

```py
import httpx


async def fetch(url):
    with httpx.Client() as c:
        return c.get(url)
```

```expect
6:16 py/blocking-http-call-httpx-in-async-function
```

### case: an optional annotation

```py
from typing import Optional
import httpx


async def fetch(c: Optional[httpx.Client]):
    return c.get('x')
```

```expect
6:12 py/blocking-http-call-httpx-in-async-function
```

### case: an AsyncClient

```py
import httpx


async def fetch(url):
    async with httpx.AsyncClient() as c:
        return await c.get(url)
```

```expect
(none)
```

### case: a synchronous function

```py
import httpx


def fetch(url):
    client = httpx.Client()
    return client.get(url)
```

```expect
(none)
```

### case: a method that does not block

```py
import httpx


async def f():
    client = httpx.Client()
    return client.build_request('GET', 'x')
```

```expect
(none)
```

### case: an unknown client

```py
async def fetch(client, url):
    return client.get(url)
```

```expect
(none)
```
