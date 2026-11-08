---
id: scss/double-slash-comment-empty
title: "No empty `//` comments"
languages: [scss]
fix: auto
on_edit: false
fix_reason: "Deletes a placeholder still being filled in, so save and the agent hook leave it for codelaw fix."
stability: preview
ported_from: [stylelint-scss/double-slash-comment-empty, stylelint-scss/comment-no-empty]
since: 0.3.0
---

A `//` comment holds something. An empty one is removed.

**Why:** An empty comment says nothing; it is usually left over from an edit.

```codelaw
native: scss/double-slash-comment-empty
```

## Examples

### SCSS

#### Don't

```scss
.a {
  //
  color: red;
}
```

#### Do

```scss
.a {
  color: red;
}
```

## Differences from the source

A `///` SassDoc line is not empty.
