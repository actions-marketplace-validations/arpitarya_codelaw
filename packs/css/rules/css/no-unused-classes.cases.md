# css/no-unused-classes — cases

### case: a class nothing that loads the stylesheet writes

```css files
/* src/Card.css */
.card { padding: 1rem; }
.legacy { color: red; }
// src/Card.jsx
import "./Card.css";
export const C = () => <div className="card" />;
```

```expect
src/Card.css:2:2 css/no-unused-classes
```

### case: a class written by any file under the same entry

```css files
/* src/index.css */
.container { margin: 0 auto; }
.dead { color: red; }
// src/Card.jsx
export const C = () => <div className="container" />;
// src/main.jsx
import "./index.css";
import { C } from "./Card";
C;
```

```expect
src/index.css:2:2 css/no-unused-classes
```

### case: a page's markup uses it

```css files
/* public/site.css */
.site { color: red; }
.gone { color: red; }
<!-- public/index.html -->
<link rel="stylesheet" href="site.css"><body class="site"></body>
```

```expect
public/site.css:2:2 css/no-unused-classes
```

### case: a name built at run time keeps the classes that start like it

```css files
/* src/a.css */
.btn-big { color: red; }
.nav-x { color: red; }
// src/a.jsx
import "./a.css";
export const C = (p) => <b className={`btn-${p.size}`} />;
```

```expect
src/a.css:2:2 css/no-unused-classes
```

### case: a stylesheet nothing loads is not judged

```css files
/* src/a.css */
.a { color: red; }
// src/a.jsx
export const C = () => <i />;
```

```expect
(none)
```

### case: a CSS Module and a class named by interpolation are left alone

```css files
/* src/a.module.css */
.x { color: red; }
/* src/a.scss */
$n: a;
.#{$n} { color: red; }
.y { color: red; }
// src/a.jsx
import "./a.scss";
import s from "./a.module.css";
s;
```

```expect
(none)
```

### case: `@apply` in a stylesheet names a class

```css files
/* src/a.css */
.base { color: red; }
.btn { @apply base; }
// src/a.jsx
import "./a.css";
export const C = () => <i className="btn" />;
```

```expect
(none)
```
