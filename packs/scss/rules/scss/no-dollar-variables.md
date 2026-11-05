---
id: scss/no-dollar-variables
title: "No Sass variables"
languages: [scss]
fix: manual
fix_reason: "what replaces the variable (a custom property, a literal) is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/no-dollar-variables]
since: 0.5.0
---

No `$variable` is declared: the project uses custom properties instead.

**Why:** Custom properties are visible to the browser and its tools.

```codelaw
native: scss/no-dollar-variables
```

## How to fix

1. Use a custom property.

## Examples

### SCSS

#### Don't

```scss
$c: red;
```

#### Do

```scss
a {
  --c: red;
}
```
