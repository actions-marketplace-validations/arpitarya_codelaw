---
id: css/property-allowed-list
title: "Only propertys the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the property is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the properties allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/property-allowed-list]
since: 0.5.0
---

Every property is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/property-allowed-list
```

## How to fix

1. Use an allowed property, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["color"]}
a {
  float: left;
}
```

#### Do

```css options={"list":["color"]}
a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["color"]}
a {
  float: left;
}
```

#### Do

```scss options={"list":["color"]}
a {
  color: red;
}
```
