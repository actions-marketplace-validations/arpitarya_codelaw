---
id: js/no-undeclared-classes
title: "A class a script writes is declared by a stylesheet that loads with it"
languages: [javascript, jsx, tsx]
scope: project
severity: warn
fix: manual
fix_reason: "the class to write, or the rule to add to the stylesheet, is the author's to choose"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noUndeclaredClasses]
since: 0.5.2
---

A class written in a static `className` (or `class`) string of a JSX element is declared by a global stylesheet that loads with the file. Loading with a file means what its entries reach: the stylesheets the file imports, those its importers import up to the files nothing imports (an app's `main.jsx`), a page's `<link href>`, and the stylesheets each of those `@import`s. So a global `index.css` imported by `main.jsx` counts for every component under it. Behaviour of Biome's `noUndeclaredClasses`, read from its documentation and rewritten.

**Why:** a class no stylesheet declares styles nothing; a typo (`card-titel`) or a rename that missed one use shows up only on screen.

Only a word that can be a class is read: a name made of letters, digits, `-` and `_`, so a utility framework's `md:w-1/2` or `[&>p]:x` is not. A template with an interpolation, a variable and a call are not read. The rule is silent for a file that no stylesheet loads with, and whenever something that loads with it cannot be read whole: a stylesheet of a package or another host (`@import "bootstrap/…"`, `@import url(https://…)`), a Sass partial or `@import` it cannot find, Tailwind's directives, a class name built by interpolation (`.#{$n}`), a page with an inline `<style>`. A CSS Module is not a global stylesheet: its classes are read as `styles.name` (`css-modules/no-undefined-class`).

```codelaw
graph:
  sheets:
    classes: undeclared
message: "`$CLASS` is not declared by any stylesheet that loads with `$FILE`"
```

## How to fix

1. Correct the name, or add the class to the stylesheet the component imports.
2. If a library or the page adds the class at run time, declare it in a stylesheet anyway (an empty rule is enough) or keep the file out with `[files]`.

## Examples

### JavaScript

#### Don't

```js files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.js
import "./Card.css";
export const Card = () => <div className="card"><b className="card-titel" /></div>;
```

#### Do

```js files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.js
import "./Card.css";
export const Card = () => <div className="card"><b className="card-title" /></div>;
```

### JSX

#### Don't

```jsx files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.jsx
import "./Card.css";
export const Card = () => <div className="card"><b className="card-titel" /></div>;
```

#### Do

```jsx files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.jsx
import "./Card.css";
export const Card = () => <div className="card"><b className="card-title" /></div>;
```

### TSX

#### Don't

```tsx files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.tsx
import "./Card.css";
export const Card = (): JSX.Element => <div className="card"><b className="card-titel" /></div>;
```

#### Do

```tsx files
/* src/Card.css */
.card { padding: 1rem; }
.card-title { color: red; }
// src/Card.tsx
import "./Card.css";
export const Card = (): JSX.Element => <div className="card"><b className="card-title" /></div>;
```

## Differences from the source

Biome's `noUndeclaredClasses` reads the stylesheets one file imports and reports every class they do not declare. This rule reads what loads with the file (its entries' reach), so a global `index.css` loaded by the app counts, and it stays silent when any of it cannot be read. HTML `class` attributes are not read: findings in markup belong to the `html` pack, and no rule there reads a project graph yet.
