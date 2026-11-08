---
id: css/selector-class-pattern
title: "Class selectors are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in every stylesheet, page and script that uses the class, as one fix; a guess: a use in markup or a string literal may be another word, so the rename is a diff to accept (SR-MULTI-FILE-FIXES)"
stability: preview
options:
  pattern: { type: "string", default: "^([a-z][a-z0-9]*)(-[a-z0-9]+)*$", description: "a regular expression names must match" }
ported_from: [stylelint/selector-class-pattern, stylelint-scss/selector-class-pattern]
since: 0.1.0
---

Class names in selectors match a pattern, kebab-case by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/selector-class-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the class in its stylesheet and in every page and script that uses it (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename the class and its users by hand.

## Examples

### CSS

#### Don't

```css
.navBar {
  color: red;
}
```

#### Do

```css
.nav-bar {
  color: red;
}
```

#### Don't

```css
/* src/nav.css */
.navBar {
  color: red;
}
<!-- index.html -->
<nav class="navBar main"></nav>
// src/nav.js
document.querySelector(".navBar").classList.add("navBar");
```

#### Do

```css
/* src/nav.css */
.nav-bar {
  color: red;
}
<!-- index.html -->
<nav class="nav-bar main"></nav>
// src/nav.js
document.querySelector(".nav-bar").classList.add("nav-bar");
```

### SCSS

#### Don't

```scss
.navBar {
  color: red;
}
```

#### Do

```scss
.nav-bar {
  color: red;
}
```
