---
id: css/declaration-property-value-disallowed-list
title: "No value the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the value is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "property → the values (`/re/` for a regular expression)" }
ported_from: [stylelint/declaration-property-value-disallowed-list]
since: 0.5.0
---

No value named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/declaration-property-value-disallowed-list
```

## How to fix

1. Use an other value, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":{"position":["fixed"]}}
a {
  position: fixed;
}
```

#### Do

```css options={"list":{"position":["fixed"]}}
a {
  position: relative;
}
```

### SCSS

#### Don't

```scss options={"list":{"position":["fixed"]}}
a {
  position: fixed;
}
```

#### Do

```scss options={"list":{"position":["fixed"]}}
a {
  position: relative;
}
```
