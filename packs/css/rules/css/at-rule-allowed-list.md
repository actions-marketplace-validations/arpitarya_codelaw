---
id: css/at-rule-allowed-list
title: "Only at-rules the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the at-rule is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the at-rule names allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/at-rule-allowed-list]
since: 0.5.0
---

Every at-rule is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/at-rule-allowed-list
```

## How to fix

1. Use an allowed at-rule, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["media"]}
@supports (display: grid) {
  a {
    color: red;
  }
}
```

#### Do

```css options={"list":["media"]}
@media (min-width: 1px) {
  a {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss options={"list":["media"]}
@supports (display: grid) {
  a {
    color: red;
  }
}
```

#### Do

```scss options={"list":["media"]}
@media (min-width: 1px) {
  a {
    color: red;
  }
}
```
