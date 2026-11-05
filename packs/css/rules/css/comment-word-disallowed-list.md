---
id: css/comment-word-disallowed-list
title: "No word the project disallows in comments"
languages: [css, scss]
fix: manual
fix_reason: "the comment's wording is the author's to write"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the words disallowed (`/re/` for a regular expression)" }
ported_from: [stylelint/comment-word-disallowed-list]
since: 0.5.0
---

A comment holds no word named in `list`. With no option set, the rule does nothing.

**Why:** A project keeps markers like `FIXME` out of shipped stylesheets.

```codelaw
native: css/comment-word-disallowed-list
```

## How to fix

1. Reword the comment, or do what it marks.

## Examples

### CSS

#### Don't

```css options={"list":["FIXME"]}
/* FIXME colours */
a {
  color: red;
}
```

#### Do

```css options={"list":["FIXME"]}
/* Colours */
a {
  color: red;
}
```

### SCSS

#### Don't

```scss options={"list":["FIXME"]}
/* FIXME colours */
a {
  color: red;
}
```

#### Do

```scss options={"list":["FIXME"]}
/* Colours */
a {
  color: red;
}
```
