---
id: css/unit-disallowed-list
title: "No unit the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the unit is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the units disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/unit-disallowed-list]
since: 0.5.0
---

No unit named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/unit-disallowed-list
```

## How to fix

1. Use an other unit, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["em"]}
a {
  width: 2em;
}
```

#### Do

```css options={"list":["em"]}
a {
  width: 2px;
}
```

### SCSS

#### Don't

```scss options={"list":["em"]}
a {
  width: 2em;
}
```

#### Do

```scss options={"list":["em"]}
a {
  width: 2px;
}
```
