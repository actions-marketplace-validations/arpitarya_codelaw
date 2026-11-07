---
id: css-modules/composes-valid
title: "composes points at a class that exists"
languages: [css, scss]
fix: suggest
fix_reason: "the classes the module defines within two edits of the missing name are offered as choices, nearest first; the reviewer picks the one meant. A name with none near, or a `composes` in the wrong place, is manual"
fix_exception: value
stability: preview
since: 0.3.0
---

`composes` names classes the file or the module it points at defines, and stands in a rule whose selector is one top-level class.

**Why:** a bundler resolves `composes` when it builds, and what it cannot resolve is an error at build time, or a class that quietly does nothing.

It reports `composes: a from "./b.module.css"` when `b.module.css` does not exist or defines no `.a`, a local `composes: a` with no `.a` in the file, and a `composes` inside a nested rule, an at-rule, or a rule whose selector is not one class. `from global` and a package path (`from "lib/x.css"`) are not judged.

```codelaw
native: css-modules/composes-valid
```

## How to fix

1. Correct the class name or the path.
2. Move `composes` into the rule whose selector is the one class that composes.

## Examples

### CSS

#### Don't

```css files
/* src/button.module.css */
.primary { composes: bse from "./base.module.css"; }
/* src/base.module.css */
.base { color: red; }
```

#### Fixed

```css files
/* src/button.module.css */
.primary { composes: base from "./base.module.css"; }
/* src/base.module.css */
.base { color: red; }
```

#### Do

```css files
/* src/button.module.css */
.primary { composes: base from "./base.module.css"; }
/* src/base.module.css */
.base { color: red; }
```

### SCSS

#### Don't

```scss files
/* src/button.module.scss */
.base { color: red; }
.primary { composes: bsae; }
```

#### Fixed

```scss files
/* src/button.module.scss */
.base { color: red; }
.primary { composes: base; }
```

#### Do

```scss files
/* src/button.module.scss */
.base { color: red; }
.primary { composes: base; }
```
