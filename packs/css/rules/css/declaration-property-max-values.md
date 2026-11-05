---
id: css/declaration-property-max-values
title: "No declaration with more values than the project allows"
languages: [css, scss]
fix: manual
fix_reason: "which values to drop is the author's call"
fix_exception: value
stability: preview
options:
  max: { type: "object", default: {}, description: "property → the most values it may have" }
ported_from: [stylelint/declaration-property-max-values]
since: 0.5.0
---

A declaration of a property named in `max` has at most that many values: `{ "margin": 2 }`. With no option set, the rule does nothing.

**Why:** A project keeps shorthands readable.

```codelaw
native: css/declaration-property-max-values
```

## How to fix

1. Use fewer values, or the longhand properties.

## Examples

### CSS

#### Don't

```css options={"max":{"margin":2}}
a {
  margin: 1px 2px 3px;
}
```

#### Do

```css options={"max":{"margin":2}}
a {
  margin: 1px 2px;
}
```

### SCSS

#### Don't

```scss options={"max":{"margin":2}}
a {
  margin: 1px 2px 3px;
}
```

#### Do

```scss options={"max":{"margin":2}}
a {
  margin: 1px 2px;
}
```
