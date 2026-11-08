---
id: css/container-name-pattern
title: "Container names are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in every `container-name`, `container` and `@container` of the project, as one fix; a diff to accept (SR-MULTI-FILE-FIXES)"
stability: stable
options:
  pattern: { type: "string", default: "^(--)?([a-z][a-z0-9]*)(-[a-z0-9]+)*$", description: "a regular expression names must match" }
ported_from: [stylelint/container-name-pattern]
since: 0.1.0
---

Names given in `container-name`, `container` and `@container` match a pattern, kebab-case by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/container-name-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the container and every `@container` query that uses it (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it by hand.

## Examples

### CSS

#### Don't

```css
.card {
  container-name: cardGrid;
}
```

#### Do

```css
.card {
  container-name: card-grid;
}
```

#### Don't

```css
/* src/card.css */
.card {
  container-name: cardGrid;
}
/* src/title.css */
@container cardGrid (width > 30em) {
  h2 {
    font-size: 2em;
  }
}
```

#### Do

```css
/* src/card.css */
.card {
  container-name: card-grid;
}
/* src/title.css */
@container card-grid (width > 30em) {
  h2 {
    font-size: 2em;
  }
}
```

### SCSS

#### Don't

```scss
.card {
  container-name: cardGrid;
}
```

#### Do

```scss
.card {
  container-name: card-grid;
}
```
