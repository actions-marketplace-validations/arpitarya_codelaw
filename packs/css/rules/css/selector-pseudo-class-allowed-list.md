---
id: css/selector-pseudo-class-allowed-list
title: "Only pseudo-classs the project allows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the pseudo-class is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the pseudo-classes allowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-pseudo-class-allowed-list]
since: 0.5.0
---

Every pseudo-class is in the `list` option. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-pseudo-class-allowed-list
```

## How to fix

1. Use an allowed pseudo-class, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["hover"]}
a:focus {
  color: red;
}
```

#### Do

```css options={"list":["hover"]}
a:hover {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["hover"]}
a:focus {
  color: red;
}
```

#### Do

```scss options={"list":["hover"]}
a:hover {
  color: red;
}
```
