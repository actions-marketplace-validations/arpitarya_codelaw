---
id: css/function-allowed-list
title: "Only functions the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the function is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the function names allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/function-allowed-list]
since: 0.5.0
---

Every function is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/function-allowed-list
```

## How to fix

1. Use an allowed function, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["rgb"]}
a {
  color: hsl(0 0% 0%);
}
```

#### Do

```css options={"list":["rgb"]}
a {
  color: rgb(0 0 0);
}
```

### SCSS

#### Don't

```scss options={"list":["rgb"]}
a {
  color: hsl(0 0% 0%);
}
```

#### Do

```scss options={"list":["rgb"]}
a {
  color: rgb(0 0 0);
}
```
