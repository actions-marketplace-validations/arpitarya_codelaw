---
id: css/keyframes-name-pattern
title: "Keyframes names are kebab-case"
languages: [css, scss]
fix: suggest
fix_reason: "renamed to kebab-case in its `@keyframes` and in every `animation` and `animation-name` of the project, as one fix; a diff to accept, as a stylesheet may build the name another way (SR-MULTI-FILE-FIXES)"
stability: stable
options:
  pattern: { type: "string", default: "^([a-z][a-z0-9]*)(-[a-z0-9]+)*$", description: "a regular expression names must match" }
ported_from: [stylelint/keyframes-name-pattern]
since: 0.1.0
---

The name of each `@keyframes` matches a pattern, kebab-case by default.

**Why:** A consistent naming scheme makes names predictable across a codebase.

```codelaw
native: css/keyframes-name-pattern
```

## How to fix

1. Accept the rename: `codelaw fix --unsafe` renames the keyframes and every `animation` and `animation-name` that uses them (the diff names each file).
2. A quoted name, a pattern that isn't kebab-case, or a new name already taken offers no rename: rename it by hand.

## Examples

### CSS

#### Don't

```css
@keyframes fadeIn {
  from {
    opacity: 0;
  }
}
```

#### Do

```css
@keyframes fade-in {
  from {
    opacity: 0;
  }
}
```

#### Don't

```css
/* src/fade.css */
@keyframes fadeIn {
  from {
    opacity: 0;
  }
}
/* src/card.css */
.card {
  animation: fadeIn 1s;
}
```

#### Do

```css
/* src/fade.css */
@keyframes fade-in {
  from {
    opacity: 0;
  }
}
/* src/card.css */
.card {
  animation: fade-in 1s;
}
```

### SCSS

#### Don't

```scss
@keyframes fadeIn {
  from {
    opacity: 0;
  }
}
```

#### Do

```scss
@keyframes fade-in {
  from {
    opacity: 0;
  }
}
```
