---
id: css/declaration-property-value-allowed-list
title: "Only values the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the value is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "property → the values (`/re/` for a regular expression)" }
ported_from: [stylelint/declaration-property-value-allowed-list]
since: 0.5.0
---

Every value is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/declaration-property-value-allowed-list
```

## How to fix

1. Use an allowed value, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":{"position":["relative"]}}
a {
  position: fixed;
}
```

#### Do

```css options={"list":{"position":["relative"]}}
a {
  position: relative;
}
```

### SCSS

#### Don't

```scss options={"list":{"position":["relative"]}}
a {
  position: fixed;
}
```

#### Do

```scss options={"list":{"position":["relative"]}}
a {
  position: relative;
}
```
