---
id: css/no-undeclared-custom-properties
title: "A custom property a stylesheet reads is declared by one that loads with it"
languages: [css, scss]
scope: project
severity: warn
fix: manual
fix_reason: "the value of the property, or a fallback, is the author's to write"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noUndeclaredCustomProperties]
since: 0.5.2
---

A `var(--name)` without a fallback names a custom property that a stylesheet loading with this one declares (`--name: …`) or registers (`@property --name`), or that a script or page of the same scope writes (`setProperty('--name', …)`, `style="--name: …"`). Loading with a stylesheet means what its entries reach, as for `js/no-undeclared-classes`. Behaviour of Biome's `noUndeclaredCustomProperties`, read from its documentation and rewritten.

**Why:** an undeclared custom property makes the declaration invalid at computed-value time, so the property falls back to its inherited or initial value and the page loses its style without an error.

`var(--name, fallback)` is not reported. A stylesheet nothing loads is left alone (whoever loads it may declare the property), and so is one with anything that cannot be read whole in its scope: a stylesheet of a package or another host, a missing Sass partial, Tailwind's directives, a property named by interpolation (`--#{$n}`). `css/no-unknown-custom-properties` reads one stylesheet; this rule reads the project.

```codelaw
graph:
  sheets:
    properties: undeclared
message: "`$PROPERTY` is not declared by any stylesheet that loads with `$FILE`"
```

## How to fix

1. Declare the property, in `:root` or in the rule that needs it, or give `var()` a fallback.
2. If a script sets it at run time, write its name in that script (`style.setProperty('--name', …)`): any `--name` the scope's scripts and pages write counts as declared.

## Examples

### CSS

#### Don't

```css files
/* src/a.css */
.a { color: var(--brand); }
// src/main.js
import "./a.css";
```

#### Do

```css files
/* src/a.css */
.a { color: var(--brand); }
/* src/theme.css */
:root { --brand: red; }
// src/main.js
import "./a.css";
import "./theme.css";
```

### SCSS

#### Don't

```scss files
/* src/a.scss */
.a { color: var(--brand); }
// src/main.js
import "./a.scss";
```

#### Do

```scss files
/* src/a.scss */
.a { color: var(--brand); }
/* src/theme.scss */
:root { --brand: red; }
// src/main.js
import "./a.scss";
import "./theme.scss";
```

## Differences from the source

Biome's `noUndeclaredCustomProperties` reads the stylesheets one file imports. This rule reads what loads with the stylesheet through the project graph, counts what scripts and pages write, and stays silent when part of the scope cannot be read. Properties named in JSX `style` strings are not read.
