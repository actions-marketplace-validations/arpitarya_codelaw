---
id: css/function-disallowed-list
title: "No function the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the function is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the function names disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/function-disallowed-list]
since: 0.5.0
---

No function named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/function-disallowed-list
```

## How to fix

1. Use an other function, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["hsl"]}
a {
  color: hsl(0 0% 0%);
}
```

#### Do

```css options={"list":["hsl"]}
a {
  color: rgb(0 0 0);
}
```

### SCSS

#### Don't

```scss options={"list":["hsl"]}
a {
  color: hsl(0 0% 0%);
}
```

#### Do

```scss options={"list":["hsl"]}
a {
  color: rgb(0 0 0);
}
```
