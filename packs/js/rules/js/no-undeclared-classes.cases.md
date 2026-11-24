# js/no-undeclared-classes — cases

### case: a typo no stylesheet declares

```jsx files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.jsx
import "./Card.css";
export const Card = () => <div className="card card-titel"><b className={'card-title'} /></div>;
```

```expect
src/Card.jsx:2:48 js/no-undeclared-classes
```

### case: the stylesheets of the entry count for a component under it

```jsx files
/* src/index.css */
.container { margin: 0 auto; }
/* src/Card.css */
.card { padding: 1rem; }
// src/Card.jsx
import "./Card.css";
export const C = () => <div className="container card" />;
// src/App.jsx
import { C } from "./Card";
export const App = () => <C />;
// src/main.jsx
import "./index.css";
import { App } from "./App";
App;
```

```expect
(none)
```

### case: a class the stylesheet's own import declares

```jsx files
/* src/base.css */
.btn { color: red; }
/* src/Card.css */
@import "./base.css";
.card { padding: 1rem; }
// src/Card.jsx
import "./Card.css";
export const C = () => <div className="btn card nope" />;
```

```expect
src/Card.jsx:2:49 js/no-undeclared-classes
```

### case: a Sass partial found by `@use`

```jsx files
/* src/_vars.scss */
.p { color: red; }
/* src/a.scss */
@use "vars";
.a { color: red; }
// src/m.jsx
import "./a.scss";
export const C = () => <i className="a p q" />;
```

```expect
src/m.jsx:2:42 js/no-undeclared-classes
```

### case: no stylesheet loads with the file

```jsx files
/* src/a.css */
.a { color: red; }
// src/a.jsx
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: a package's stylesheet imported by the stylesheet is not read

```jsx files
/* src/a.css */
@import "bootstrap/dist/css/bootstrap.css";
.a { color: red; }
// src/a.jsx
import "./a.css";
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: another host's stylesheet is not read

```jsx files
/* src/a.css */
@import url(https://cdn.example.com/x.css);
.a { color: red; }
// src/a.jsx
import "./a.css";
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: Tailwind's directives are not read

```jsx files
/* src/a.css */
@tailwind base;
.a { color: red; }
// src/a.jsx
import "./a.css";
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: a Sass partial that is not found is not read

```jsx files
/* src/a.scss */
@use "missing";
.a { color: red; }
// src/a.jsx
import "./a.scss";
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: a package's stylesheet imported by the script

```jsx files
/* src/a.css */
.a { color: red; }
// src/a.jsx
import "./a.css";
import "bootstrap/dist/css/bootstrap.css";
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: a class name built by interpolation hides the stylesheet's classes

```jsx files
/* src/a.scss */
$n: a;
.#{$n} { color: red; }
// src/a.jsx
import "./a.scss";
export const C = () => <div className="nope" />;
```

```expect
(none)
```

### case: only static strings and words shaped like a class are read

```jsx files
/* src/a.css */
.a { color: red; }
// src/a.jsx
import "./a.css";
const x = "b";
export const C = () => <div className={`a ${x}`} class={x} className="md:w-1/2 [&>p]:x a" />;
```

```expect
(none)
```

### case: a CSS Module is not a global stylesheet

```jsx files
/* src/a.module.css */
.x { color: red; }
// src/a.jsx
import s from "./a.module.css";
export const C = () => <div className="x" />;
```

```expect
(none)
```

### case: a page that links a stylesheet and loads the script

```jsx files
/* public/site.css */
.site { color: red; }
/* public/own.css */
.own { color: blue; }
<!-- public/index.html -->
<link rel="stylesheet" href="site.css"><script src="app.jsx"></script>
// public/app.jsx
import "./own.css";
export const C = () => <div className="site own gone" />;
```

```expect
public/app.jsx:2:49 js/no-undeclared-classes
```

### case: a page with an inline style is not read

```jsx files
/* public/own.css */
.own { color: blue; }
<!-- public/index.html -->
<style>.x { color: red; }</style><script src="app.jsx"></script>
// public/app.jsx
import "./own.css";
export const C = () => <div className="own gone" />;
```

```expect
(none)
```
