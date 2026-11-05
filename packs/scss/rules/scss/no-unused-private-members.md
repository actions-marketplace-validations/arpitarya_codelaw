---
id: scss/no-unused-private-members
title: "No private member that is never used"
languages: [scss]
fix: manual
fix_reason: "whether the member is dead or should be used is the author's call"
fix_exception: restructure
stability: preview
ported_from: [stylelint-scss/no-unused-private-members]
since: 0.5.0
---

A private (`-` or `_`) variable, mixin, function or placeholder is used in the file that declares it.

**Why:** Nobody else can use it.

```codelaw
native: scss/no-unused-private-members
```

## How to fix

1. Remove it, or use it.

## Examples

### SCSS

#### Don't

```scss
$-unused: 1px;
a {
  color: red;
}
```

#### Do

```scss
$-gap: 1px;
a {
  margin: $-gap;
}
```
