---
id: css/selector-pseudo-element-allowed-list
title: "Only pseudo-elements the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the pseudo-element is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the pseudo-elements allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-pseudo-element-allowed-list]
since: 0.5.0
---

Every pseudo-element is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-pseudo-element-allowed-list
```

## How to fix

1. Use an allowed pseudo-element, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["before"]}
a::after {
  color: red;
}
```

#### Do

```css options={"list":["before"]}
a::before {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["before"]}
a::after {
  color: red;
}
```

#### Do

```scss options={"list":["before"]}
a::before {
  color: red;
}
```
