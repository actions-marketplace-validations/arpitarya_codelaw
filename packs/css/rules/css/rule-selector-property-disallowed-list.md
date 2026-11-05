---
id: css/rule-selector-property-disallowed-list
title: "No property the project disallows for a selector"
languages: [css, scss]
fix: manual
fix_reason: "where the property belongs instead is the author's call"
fix_exception: value
stability: preview
options:
  list: { type: "object", default: {}, description: "selector → the properties it may not declare" }
ported_from: [stylelint/rule-selector-property-disallowed-list]
since: 0.5.0
---

A rule whose selector is named in `list` doesn't declare the properties listed for it: `{ "a": ["color"] }`. With no option set, the rule does nothing.

**Why:** A project reserves some properties for its components.

```codelaw
native: css/rule-selector-property-disallowed-list
```

## How to fix

1. Move the declaration to where it belongs.

## Examples

### CSS

#### Don't

```css options={"list":{"a":["color"]}}
a {
  color: red;
}
```

#### Do

```css options={"list":{"a":["color"]}}
a {
  margin: 0;
}
```

### SCSS

#### Don't

```scss options={"list":{"a":["color"]}}
a {
  color: red;
}
```

#### Do

```scss options={"list":{"a":["color"]}}
a {
  margin: 0;
}
```
