---
id: scss/at-function-pattern
title: "Function names follow the project's pattern"
languages: [scss]
fix: manual
fix_reason: "renaming the function must also rename its callers, possibly in other files"
fix_exception: value
stability: preview
options:
  pattern: { type: "string", default: "^[a-z][a-z0-9]*(-[a-z0-9]+)*$", description: "a regular expression names must match" }
ported_from: [stylelint-scss/at-function-pattern]
since: 0.5.0
---

A `@function`'s name matches `pattern`, kebab-case by default.

**Why:** Consistent names are predictable.

```codelaw
native: scss/at-function-pattern
```

## How to fix

1. Rename the function and its calls.

## Examples

### SCSS

#### Don't

```scss
@function myFn($a) {
  @return $a;
}
```

#### Do

```scss
@function my-fn($a) {
  @return $a;
}
```
