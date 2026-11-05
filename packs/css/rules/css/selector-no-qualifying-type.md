---
id: css/selector-no-qualifying-type
title: "No class, id or attribute qualified by a type"
languages: [css, scss]
fix: suggest
fix_reason: "dropping the type changes what the selector matches and its specificity"
stability: preview
ported_from: [stylelint/selector-no-qualifying-type]
since: 0.5.0
---

A class, id or attribute selector isn't joined to a type selector: `a.link` is `.link`.

**Why:** The type adds specificity and ties the style to one element.

```codelaw
native: css/selector-no-qualifying-type
```

## How to fix

1. Drop the type selector.

## Examples

### CSS

#### Don't

```css
a.link {
  color: red;
}
```

#### Do

```css
.link {
  color: red;
}
```

### SCSS

#### Don't

```scss
a.link {
  color: red;
}
```

#### Do

```scss
.link {
  color: red;
}
```
