# css/no-undeclared-custom-properties — cases

### case: a property nobody declares

```css files
/* src/a.css */
.a { color: var(--brand); border: 1px solid var(--edge, red); }
// src/main.js
import "./a.css";
```

```expect
src/a.css:1:17 css/no-undeclared-custom-properties
```

### case: declared in another stylesheet of the scope, or registered

```css files
/* src/a.css */
.a { color: var(--brand); margin: var(--gap); }
/* src/b.css */
:root { --brand: red; }
@property --gap { syntax: "<length>"; inherits: false; initial-value: 0; }
// src/main.js
import "./a.css";
import "./b.css";
```

```expect
(none)
```

### case: a script sets it at run time

```css files
/* src/a.css */
.a { color: var(--brand); }
// src/main.js
import "./a.css";
document.documentElement.style.setProperty("--brand", "red");
```

```expect
(none)
```

### case: a page sets it in a style attribute

```css files
/* src/a.css */
.a { color: var(--brand); }
<!-- src/index.html -->
<link rel="stylesheet" href="a.css"><body style="--brand: red"></body>
```

```expect
(none)
```

### case: a Sass partial declares it

```css files
/* src/_vars.scss */
:root { --brand: red; }
/* src/a.scss */
@use "vars";
.a { color: var(--brand); }
// src/main.js
import "./a.scss";
```

```expect
(none)
```

### case: a stylesheet nothing loads is not judged

```css files
/* src/a.css */
.a { color: var(--brand); }
```

```expect
(none)
```

### case: a stylesheet imported from a package path is not read

```css files
/* src/a.css */
@import "theme/vars.css";
.a { color: var(--brand); }
// src/main.js
import "./a.css";
```

```expect
(none)
```

### case: a property named by interpolation is not read

```css files
/* src/a.scss */
$n: x;
:root { --#{$n}: 1; }
.a { color: var(--brand); }
// src/main.js
import "./a.scss";
```

```expect
(none)
```
