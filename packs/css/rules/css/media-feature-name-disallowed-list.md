---
id: css/media-feature-name-disallowed-list
title: "No media feature the project disallows"
languages: [css, scss]
fix: manual
fix_reason: "what the project uses instead of the media feature is its own convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the media features disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/media-feature-name-disallowed-list]
since: 0.5.0
---

No media feature named in the `list` option is used. With no option set, the rule does nothing.

**Why:** A project keeps to the subset of CSS it has agreed on.

```codelaw
native: css/media-feature-name-disallowed-list
```

## How to fix

1. Use an other media feature, or change the list.

## Examples

### CSS

#### Don't

```css options={"list":["max-width"]}
@media (max-width: 1px) {
  a {
    color: red;
  }
}
```

#### Do

```css options={"list":["max-width"]}
@media (min-width: 1px) {
  a {
    color: red;
  }
}
```

### SCSS

#### Don't

```scss options={"list":["max-width"]}
@media (max-width: 1px) {
  a {
    color: red;
  }
}
```

#### Do

```scss options={"list":["max-width"]}
@media (min-width: 1px) {
  a {
    color: red;
  }
}
```
