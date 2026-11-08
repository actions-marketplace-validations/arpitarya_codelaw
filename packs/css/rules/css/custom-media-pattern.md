---
id: css/custom-media-pattern
title: "Custom media names are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in its `@custom-media` and in every `@media` of the project that uses it, as one fix; a diff to accept (SR-MULTI-FILE-FIXES)"
stability: stable
options:
  pattern: { type: "string", default: "^([a-z][a-z0-9]*)(-[a-z0-9]+)*$", description: "a regular expression names must match (without the leading `--`)" }
ported_from: [stylelint/custom-media-pattern]
since: 0.1.0
---

The name of each `@custom-media` (after `--`) matches a pattern, kebab-case by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/custom-media-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the custom media and every query that uses it (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it by hand.

## Examples

### CSS

#### Don't

```css
@custom-media --smallViewport (width <= 30em);
```

#### Do

```css
@custom-media --small-viewport (width <= 30em);
```

#### Don't

```css
/* src/media.css */
@custom-media --smallViewport (width <= 30em);
/* src/nav.css */
@media (--smallViewport) {
  nav {
    display: none;
  }
}
```

#### Do

```css
/* src/media.css */
@custom-media --small-viewport (width <= 30em);
/* src/nav.css */
@media (--small-viewport) {
  nav {
    display: none;
  }
}
```

### SCSS

#### Don't

```scss
@custom-media --smallViewport (width <= 30em);
```

#### Do

```scss
@custom-media --small-viewport (width <= 30em);
```
