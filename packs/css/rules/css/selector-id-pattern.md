---
id: css/selector-id-pattern
title: "Id selectors are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in every stylesheet, page and script that uses the id (`#id`, `id`, `href=\"#…\"`, `for`, `aria-*`, `getElementById`), as one fix; a guess: a use in markup or a string literal may be another word, so the rename is a diff to accept (SR-MULTI-FILE-FIXES)"
stability: stable
options:
  pattern: { type: "string", default: "^([a-z][a-z0-9]*)(-[a-z0-9]+)*$", description: "a regular expression names must match" }
ported_from: [stylelint/selector-id-pattern]
since: 0.1.0
---

Ids in selectors match a pattern, kebab-case by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/selector-id-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the id in its stylesheet and in every page and script that uses it (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename the id and its users by hand.

## Examples

### CSS

#### Don't

```css
#mainNav {
  color: red;
}
```

#### Do

```css
#main-nav {
  color: red;
}
```

#### Don't

```css
/* src/a.css */
#mainNav {
  color: red;
}
<!-- index.html -->
<nav id="mainNav"></nav>
<a href="#mainNav">Skip</a>
// src/a.js
document.getElementById("mainNav").focus();
```

#### Do

```css
/* src/a.css */
#main-nav {
  color: red;
}
<!-- index.html -->
<nav id="main-nav"></nav>
<a href="#main-nav">Skip</a>
// src/a.js
document.getElementById("main-nav").focus();
```

### SCSS

#### Don't

```scss
#mainNav {
  color: red;
}
```

#### Do

```scss
#main-nav {
  color: red;
}
```
