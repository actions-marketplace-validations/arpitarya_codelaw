# py/missing-required-import — cases

### case: the import is already there

```py options={"required_imports":["from\u0020__future__\u0020import\u0020annotations"]}
"""Doc."""

from __future__ import annotations

import os
```

```expect
(none)
```

### case: below a docstring

```py options={"required_imports":["from\u0020__future__\u0020import\u0020annotations"]}
"""Doc."""

import os
```

```expect
1:1 py/missing-required-import
```

```fixed
"""Doc."""

from __future__ import annotations

import os
```

### case: a file of only a docstring

```py options={"required_imports":["from\u0020__future__\u0020import\u0020annotations"]}
"""Doc."""
```

```expect
(none)
```
