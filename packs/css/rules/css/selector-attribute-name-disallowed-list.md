---
id: css/selector-attribute-name-disallowed-list
title: "No attribute selector the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the attribute selector is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the attribute names disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/selector-attribute-name-disallowed-list]
since: 0.5.0
---

No attribute selector named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/selector-attribute-name-disallowed-list
```

## How to fix

1. Use an other attribute selector, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["id"]}
[id] {
  color: red;
}
```

#### Do

```css options={"list":["id"]}
[title] {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["id"]}
[id] {
  color: red;
}
```

#### Do

```scss options={"list":["id"]}
[title] {
  color: red;
}
```
