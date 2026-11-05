---
id: css/selector-attribute-operator-allowed-list
title: "Only attribute operators the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the attribute operator is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the operators allowed (`=`, `~=`, `|=`, `^=`, `$=`, `*=`) (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-attribute-operator-allowed-list]
since: 0.5.0
---

Every attribute operator is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-attribute-operator-allowed-list
```

## How to fix

1. Use an allowed attribute operator, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["="]}
[title^="x"] {
  color: red;
}
```

#### Do

```css options={"list":["="]}
[title="x"] {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["="]}
[title^="x"] {
  color: red;
}
```

#### Do

```scss options={"list":["="]}
[title="x"] {
  color: red;
}
```
