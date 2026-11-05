---
id: css/selector-nested-pattern
title: "Nested selectors follow the project's pattern"
languages: [css, scss]
fix: manual
fix_reason: "the selector is the author's to rewrite"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", description: "a regular expression nested selectors must match" }
ported_from: [stylelint/selector-nested-pattern]
since: 0.5.0
---

A nested selector matches the regular expression in `pattern`. With no option set, the rule does nothing.

**Why:** A project allows only some kinds of nesting (`^&:` for states).

```codelaw
native: css/selector-nested-pattern
```

## How to fix

1. Rewrite the nested selector, or un-nest it.

## Examples

### CSS

#### Don't

```css options={"pattern":"^&"}
a {
  .b {
    color: red;
  }
}
```

#### Do

```css options={"pattern":"^&"}
a {
  &:hover {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss options={"pattern":"^&"}
a {
  .b {
    color: red;
  }
}
```

#### Do

```scss options={"pattern":"^&"}
a {
  &:hover {
    color: red;
  }
}
```
