# py/deprecated-attribute — cases

### case: an attribute at the target that deprecates it

```py python=3.10
import ssl

context = ssl.SSLContext(ssl.PROTOCOL_TLS)
```

```expect
3:26 py/deprecated-attribute
```

### case: target before the deprecation

```py python=3.9
import ssl

context = ssl.SSLContext(ssl.PROTOCOL_TLS)
```

```expect
(none)
```

### case: no target

```py
import ssl

ssl.PROTOCOL_TLS
```

```expect
(none)
```

### case: from-import

```py python=3.7
from ssl import OP_NO_TLSv1, PROTOCOL_TLS_CLIENT
```

```expect
1:17 py/deprecated-attribute
```

### case: a module imported under another name

```py python=3.10
import ssl as tls

tls.OP_NO_SSLv3
```

```expect
3:1 py/deprecated-attribute
```

### case: a local object with the same attribute name

```py python=3.12
class Options:
    PROTOCOL_TLS = 1

Options.PROTOCOL_TLS
ssl = Options
ssl.PROTOCOL_TLS
```

```expect
(none)
```

### case: the replacement is fine

```py python=3.12
import ssl

ssl.PROTOCOL_TLS_CLIENT
```

```expect
(none)
```
