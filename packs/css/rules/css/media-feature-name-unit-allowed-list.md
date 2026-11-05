---
id: css/media-feature-name-unit-allowed-list
title: "Media features use the units the project allows"
languages: [css, scss]
fix: manual
fix_reason: "the unit to use is the project's convention"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "media feature → the units it may use" }
ported_from: [stylelint/media-feature-name-unit-allowed-list]
since: 0.5.0
---

A media feature named in `list` uses only its listed units: `{ "min-width": ["em"] }`. With no option set, the rule does nothing.

**Why:** Breakpoints in one unit compare easily.

```codelaw
native: css/media-feature-name-unit-allowed-list
```

## How to fix

1. Convert the value to an allowed unit.

## Examples

### CSS

#### Don't

```css options={"list":{"min-width":["em"]}}
@media (min-width: 10px) {
  a {
    color: red;
  }
}
```

#### Do

```css options={"list":{"min-width":["em"]}}
@media (min-width: 10em) {
  a {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss options={"list":{"min-width":["em"]}}
@media (min-width: 10px) {
  a {
    color: red;
  }
}
```

#### Do

```scss options={"list":{"min-width":["em"]}}
@media (min-width: 10em) {
  a {
    color: red;
  }
}
```
