---
id: css/selector-combinator-allowed-list
title: "Only combinators the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the combinator is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the combinators allowed (`>`, `+`, `~`, a space) (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-combinator-allowed-list]
since: 0.5.0
---

Every combinator is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-combinator-allowed-list
```

## How to fix

1. Use an allowed combinator, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":[">"]}
a + b {
  color: red;
}
```

#### Do

```css options={"list":[">"]}
a > b {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":[">"]}
a + b {
  color: red;
}
```

#### Do

```scss options={"list":[">"]}
a > b {
  color: red;
}
```
