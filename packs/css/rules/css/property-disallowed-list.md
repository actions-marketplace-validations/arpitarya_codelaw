---
id: css/property-disallowed-list
title: "No property the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the property is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the properties disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/property-disallowed-list]
since: 0.5.0
---

No property named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/property-disallowed-list
```

## How to fix

1. Use an other property, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["float"]}
a {
  float: left;
}
```

#### Do

```css options={"list":["float"]}
a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["float"]}
a {
  float: left;
}
```

#### Do

```scss options={"list":["float"]}
a {
  color: red;
}
```
