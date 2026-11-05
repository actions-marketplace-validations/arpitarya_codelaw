---
id: css/selector-pseudo-class-disallowed-list
title: "No pseudo-class the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the pseudo-class is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the pseudo-classes disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-pseudo-class-disallowed-list]
since: 0.5.0
---

No pseudo-class named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-pseudo-class-disallowed-list
```

## How to fix

1. Use an other pseudo-class, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["focus"]}
a:focus {
  color: red;
}
```

#### Do

```css options={"list":["focus"]}
a:hover {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["focus"]}
a:focus {
  color: red;
}
```

#### Do

```scss options={"list":["focus"]}
a:hover {
  color: red;
}
```
