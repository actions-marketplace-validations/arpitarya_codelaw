---
id: css/selector-attribute-operator-disallowed-list
title: "No attribute operator the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the attribute operator is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the operators disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-attribute-operator-disallowed-list]
since: 0.5.0
---

No attribute operator named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-attribute-operator-disallowed-list
```

## How to fix

1. Use an other attribute operator, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["*="]}
[title*="x"] {
  color: red;
}
```

#### Do

```css options={"list":["*="]}
[title="x"] {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["*="]}
[title*="x"] {
  color: red;
}
```

#### Do

```scss options={"list":["*="]}
[title="x"] {
  color: red;
}
```
