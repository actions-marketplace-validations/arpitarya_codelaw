---
id: css/custom-property-pattern
title: "Custom property names are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in every declaration, `var()`, `style` attribute and script string that names it, as one fix; a guess: a use in markup or a string literal may be another word, so the rename is a diff to accept (SR-MULTI-FILE-FIXES)"
stability: preview
options:
  pattern: { type: "string", default: "^([a-z][a-z0-9]*)(-[a-z0-9]+)*$", description: "a regular expression names must match (without the leading `--`)" }
ported_from: [stylelint/custom-property-pattern]
since: 0.1.0
---

Custom property names (after `--`), in declarations, `var()` and `@property`, match a pattern, kebab-case by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/custom-property-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the custom property everywhere it is declared or read (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it and its readers by hand.

## Examples

### CSS

#### Don't

```css
:root {
  --brandColor: red;
}
```

#### Do

```css
:root {
  --brand-color: red;
}
```

#### Don't

```css
/* src/a.css */
:root {
  --brandColor: red;
}
.a {
  color: var(--brandColor);
}
// src/a.js
document.body.style.setProperty("--brandColor", "blue");
```

#### Do

```css
/* src/a.css */
:root {
  --brand-color: red;
}
.a {
  color: var(--brand-color);
}
// src/a.js
document.body.style.setProperty("--brand-color", "blue");
```

### SCSS

#### Don't

```scss
:root {
  --brandColor: red;
}
```

#### Do

```scss
:root {
  --brand-color: red;
}
```
