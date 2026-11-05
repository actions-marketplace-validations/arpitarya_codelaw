---
id: css/media-feature-name-value-allowed-list
title: "Media features use the values the project allows"
languages: [css, scss]
fix: manual
fix_reason: "the value to use is the project's convention"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "media feature → the values it may take" }
ported_from: [stylelint/media-feature-name-value-allowed-list]
since: 0.5.0
---

A media feature named in `list` takes only its listed values: `{ "min-width": ["768px"] }`. With no option set, the rule does nothing.

**Why:** A project keeps to its breakpoints.

```codelaw
native: css/media-feature-name-value-allowed-list
```

## How to fix

1. Use one of the project's values.

## Examples

### CSS

#### Don't

```css options={"list":{"min-width":["768px"]}}
@media (min-width: 700px) {
  a {
    color: red;
  }
}
```

#### Do

```css options={"list":{"min-width":["768px"]}}
@media (min-width: 768px) {
  a {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss options={"list":{"min-width":["768px"]}}
@media (min-width: 700px) {
  a {
    color: red;
  }
}
```

#### Do

```scss options={"list":{"min-width":["768px"]}}
@media (min-width: 768px) {
  a {
    color: red;
  }
}
```
