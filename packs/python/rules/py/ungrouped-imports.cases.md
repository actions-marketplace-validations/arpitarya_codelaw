# py/ungrouped-imports — cases

### case: a package split by another

```py
import logging
import os
import logging.handlers
```

```expect
3:1 py/ungrouped-imports
```

### case: from-imports are their own set

```py
import logging
from os import pardir
from os import sep
from os.path import join
```

```expect
(none)
```

### case: a from-import split

```py
from os import pardir
import sys
from os import sep
```

```expect
3:1 py/ungrouped-imports
```

### case: a package split inside a try

```py
import logging
import sys
try:
    import logging.config
except ImportError:
    pass
```

```expect
4:5 py/ungrouped-imports
```

### case: together

```py
import logging
import logging.handlers
import os
```

```expect
(none)
```

### case: TYPE_CHECKING

```py
import logging
import os
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import logging.handlers
```

```expect
(none)
```

### case: a platform guard

```py
import sys
import re
if sys.platform == "win32":
    import re as win_re
else:
    import re as posix_re
```

```expect
(none)
```

### case: third-party and standard-library imports

```py
import six
import logging
import astroid
import os
```

```expect
(none)
```

### case: a function body

```py
import logging
import os


def f():
    import logging.handlers
```

```expect
(none)
```
