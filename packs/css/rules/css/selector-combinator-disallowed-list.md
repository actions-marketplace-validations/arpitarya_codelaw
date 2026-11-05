---
id: css/selector-combinator-disallowed-list
title: "No combinator the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the combinator is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the combinators disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-combinator-disallowed-list]
since: 0.5.0
---

No combinator named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-combinator-disallowed-list
```

## How to fix

1. Use an other combinator, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["+"]}
a + b {
  color: red;
}
```

#### Do

```css options={"list":["+"]}
a > b {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["+"]}
a + b {
  color: red;
}
```

#### Do

```scss options={"list":["+"]}
a > b {
  color: red;
}
```
