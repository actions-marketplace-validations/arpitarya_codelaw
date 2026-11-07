---
id: scss/function-no-unknown
title: "No unknown functions (SCSS)"
languages: [scss]
fix: suggest
fix_reason: "the known CSS and Sass functions within two edits are offered as choices, nearest first; a name with none near is manual"
fix_exception: value
stability: preview
options:
  ignoreFunctions: { type: "string[]", default: [], description: "functions to accept; `/regex/` for a pattern" }
ported_from: [stylelint-scss/function-no-unknown]
since: 0.3.0
---

Every function called is a CSS function, a Sass built-in (global or `module.name`), or one the file defines with `@function`. A file that loads names it cannot see — `@import`, or `@use … as *` — is not checked.

**Why:** A browser drops a declaration whose value has an unknown function, and Sass passes the call through as plain CSS, so a typo fails silently.

```codelaw
native: scss/function-no-unknown
```

## How to fix

1. Correct the function name.
2. If it comes from a library, call it through its module (`lib.name()`), or add it to `ignoreFunctions`.

## Examples

### SCSS

#### Don't

```scss
.a {
  width: percentag(0.5);
}
```

#### Fixed

```scss
.a {
  width: percentage(0.5);
}
```

#### Do

```scss
@use "sass:math";

.a {
  width: math.percentage(0.5);
}
```

## Differences from the source

Calls through a namespace (`ns.name()`) are not judged: the namespace's members are outside the file.
