---
id: css/layer-name-pattern
title: "Layer names are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in every `@layer` and `@import … layer()` of the project, as one fix; a diff to accept, as the layer order is the author's (SR-MULTI-FILE-FIXES)"
stability: stable
options:
  pattern: { type: "string", default: "^([a-z][a-z0-9]*)([.-][a-z0-9]+)*$", description: "a regular expression names must match" }
ported_from: [stylelint/layer-name-pattern]
since: 0.1.0
---

Cascade layer names, in `@layer` and in `@import … layer()`, match a pattern: kebab-case segments joined by dots, by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/layer-name-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the layer in every `@layer` and `@import … layer()` (the diff names each file).
2. A pattern that isn't kebab-case, or a new name already taken, offers no rename: rename it by hand.

## Examples

### CSS

#### Don't

```css
@layer baseStyles;
```

#### Do

```css
@layer base-styles;
```

#### Don't

```css
/* src/order.css */
@layer baseStyles, theme;
/* src/base.css */
@layer baseStyles {
  p {
    margin: 0;
  }
}
```

#### Do

```css
/* src/order.css */
@layer base-styles, theme;
/* src/base.css */
@layer base-styles {
  p {
    margin: 0;
  }
}
```

### SCSS

#### Don't

```scss
@layer baseStyles;
```

#### Do

```scss
@layer base-styles;
```
