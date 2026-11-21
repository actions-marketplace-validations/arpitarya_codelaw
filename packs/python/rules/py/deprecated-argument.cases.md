# py/deprecated-argument — cases

### case: a keyword argument at the target

```py python=3.9
import xml.etree.ElementTree as ET

ET.iterparse(path, parser=p)
```

```expect
3:20 py/deprecated-argument
```

### case: a positional argument the table knows

```py python=3.9
import xml.etree.ElementTree as ET

ET.iterparse(path, None, p)
```

```expect
3:26 py/deprecated-argument
```

### case: target before the deprecation

```py python=3.3
import xml.etree.ElementTree as ET

ET.iterparse(path, parser=p)
```

```expect
(none)
```

### case: no target

```py
import xml.etree.ElementTree as ET

ET.iterparse(path, parser=p)
```

```expect
(none)
```

### case: the argument is not passed

```py python=3.9
import xml.etree.ElementTree as ET

ET.iterparse(path, events=('end',))
```

```expect
(none)
```

### case: a star-args call: the position is uncertain

```py python=3.9
import xml.etree.ElementTree as ET

ET.iterparse(*args)
```

```expect
(none)
```

### case: a removed parameter named by keyword

```py python=3.14
from email.utils import localtime

localtime(dt, isdst=1)
```

```expect
3:15 py/deprecated-argument
```

### case: an argument of the same name to another function

```py python=3.12
def iterparse(source, parser=None): ...

iterparse(path, parser=p)
```

```expect
(none)
```

### case: a keyword-only parameter has no position

```py python=3.14
import argparse

parser = argparse.ArgumentParser()
argparse.ArgumentParser.add_argument_group(parser, 'g', prefix_chars='-')
```

```expect
4:57 py/deprecated-argument
```
