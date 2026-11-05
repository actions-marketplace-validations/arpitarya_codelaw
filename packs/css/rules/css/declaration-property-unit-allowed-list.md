---
id: css/declaration-property-unit-allowed-list
title: "Only units the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the unit is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "property → the units (`/re/` for a regular expression)" }
ported_from: [stylelint/declaration-property-unit-allowed-list]
since: 0.5.0
---

Every unit is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/declaration-property-unit-allowed-list
```

## How to fix

1. Use an allowed unit, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":{"width":["px"]}}
a {
  width: 2em;
}
```

#### Do

```css options={"list":{"width":["px"]}}
a {
  width: 2px;
}
```

### SCSS

#### Don't

```scss options={"list":{"width":["px"]}}
a {
  width: 2em;
}
```

#### Do

```scss options={"list":{"width":["px"]}}
a {
  width: 2px;
}
```
