---
id: css/selector-pseudo-element-disallowed-list
title: "No pseudo-element the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the pseudo-element is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the pseudo-elements disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-pseudo-element-disallowed-list]
since: 0.5.0
---

No pseudo-element named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-pseudo-element-disallowed-list
```

## How to fix

1. Use an other pseudo-element, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["after"]}
a::after {
  color: red;
}
```

#### Do

```css options={"list":["after"]}
a::before {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["after"]}
a::after {
  color: red;
}
```

#### Do

```scss options={"list":["after"]}
a::before {
  color: red;
}
```
