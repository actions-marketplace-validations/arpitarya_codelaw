# py/deprecated-import — cases

### case: every name moves

```py python=3.9
from collections import Mapping, Sequence as Seq
```

```expect
1:1 py/deprecated-import
```

```fixed
from collections.abc import Mapping, Sequence as Seq
```

### case: some names stay

```py python=3.9
from collections import Mapping, OrderedDict
```

```expect
1:1 py/deprecated-import
```

```fixed
from collections import OrderedDict
from collections.abc import Mapping
```

### case: an indented import with names that stay

```py python=3.9
def f():
    from collections import Mapping, OrderedDict
```

```expect
2:5 py/deprecated-import
```

```fixed
def f():
    from collections import OrderedDict
    from collections.abc import Mapping
```

### case: a parenthesised import

```py python=3.9
from collections import (
    Mapping,
    Sequence,
)
```

```expect
1:1 py/deprecated-import
```

```fixed
from collections.abc import Mapping, Sequence
```

### case: typing names move at 3.9

```py python=3.9
from typing import Sequence, Counter, Match, Any
```

```expect
1:1 py/deprecated-import
1:1 py/deprecated-import
1:1 py/deprecated-import
```

### case: typing names stay below 3.9

```py python=3.8
from typing import Sequence, Counter, Match
```

```expect
(none)
```

### case: a moved and renamed name has no edit

```py python=3.9
from typing import List
```

```expect
1:1 py/deprecated-import
```

### case: Callable moves at 3.10

```py python=3.9
from typing import Callable
```

```expect
(none)
```

### case: Callable moves at 3.10 (the target reaches it)

```py python=3.10
from typing import Callable
```

```expect
1:1 py/deprecated-import
```

### case: typing_extensions follows the target

```py python=3.10
from typing_extensions import Literal, Self
```

```expect
1:1 py/deprecated-import
```

```fixed
from typing_extensions import Self
from typing import Literal
```

### case: typing_extensions at 3.11

```py python=3.11
from typing_extensions import Literal, Self
```

```expect
1:1 py/deprecated-import
```

```fixed
from typing import Literal, Self
```

### case: a name Ruff keeps in typing_extensions

```py python=3.13
from typing_extensions import TypedDict
```

```expect
(none)
```

### case: typing.re splits on 3.9

```py python=3.8
from typing.re import Match
```

```expect
1:1 py/deprecated-import
```

```fixed
from typing import Match
```

### case: typing.re splits on 3.9 (the target reaches it)

```py python=3.9
from typing.re import Match
```

```expect
1:1 py/deprecated-import
```

```fixed
from re import Match
```

### case: a comment inside is reported without an edit

```py python=3.9
from collections import (
    Mapping,  # abc
)
```

```expect
1:1 py/deprecated-import
```

### case: star and relative imports

```py python=3.9
from collections import *
from .collections import Mapping
```

```expect
(none)
```

### case: no target

```py
from collections import Mapping
```

```expect
(none)
```
