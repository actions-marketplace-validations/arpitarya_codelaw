---
id: css/media-feature-name-allowed-list
title: "Only media features the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the media feature is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the media features allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/media-feature-name-allowed-list]
since: 0.5.0
---

Every media feature is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/media-feature-name-allowed-list
```

## How to fix

1. Use an allowed media feature, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["min-width"]}
@media (max-width: 1px) {
  a {
    color: red;
  }
}
```

#### Do

```css options={"list":["min-width"]}
@media (min-width: 1px) {
  a {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss options={"list":["min-width"]}
@media (max-width: 1px) {
  a {
    color: red;
  }
}
```

#### Do

```scss options={"list":["min-width"]}
@media (min-width: 1px) {
  a {
    color: red;
  }
}
```
