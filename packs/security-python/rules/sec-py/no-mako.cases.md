# sec-py/no-mako — cases

### case: Template

```py
from mako.template import Template

Template(src)
```

```expect
3:1 sec-py/no-mako
```

### case: qualified

```py
import mako.template

mako.template.Template(src)
```

```expect
3:1 sec-py/no-mako
```

### case: jinja Template

```py
from jinja2 import Template

Template(src)
```

```expect
(none)
```

### case: escaping already on

```py
from mako.template import Template

Template(src, default_filters=["h"])
```

```expect
(none)
```

### case: escaping is turned on

```py
from mako.template import Template

Template(src, input_encoding="utf-8")
```

```expect
3:1 sec-py/no-mako
```

```fixed
from mako.template import Template

Template(src, input_encoding="utf-8", default_filters=["str", "h"])
```
