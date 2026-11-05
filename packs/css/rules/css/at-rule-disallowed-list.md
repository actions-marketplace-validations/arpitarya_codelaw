---
id: css/at-rule-disallowed-list
title: "No at-rule the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the at-rule is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the at-rule names disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/at-rule-disallowed-list]
since: 0.5.0
---

No at-rule named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/at-rule-disallowed-list
```

## How to fix

1. Use an other at-rule, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["supports"]}
@supports (display: grid) {
  a {
    color: red;
  }
}
```

#### Do

```css options={"list":["supports"]}
a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["supports"]}
@supports (display: grid) {
  a {
    color: red;
  }
}
```

#### Do

```scss options={"list":["supports"]}
a {
  color: red;
}
```
