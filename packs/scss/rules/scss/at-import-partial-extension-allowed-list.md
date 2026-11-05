---
id: scss/at-import-partial-extension-allowed-list
title: "`@import` extensions the project allows"
languages: [scss]
fix: manual
fix_reason: "which extension the project uses is its own convention"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the extensions allowed" }
ported_from: [stylelint-scss/at-import-partial-extension-allowed-list]
since: 0.5.0
---

An `@import`'s file extension is in `list`. With no option set, the rule does nothing.

**Why:** A project loads its partials one way.

```codelaw
native: scss/at-import-partial-extension-allowed-list
```

## How to fix

1. Use an allowed extension, or none.

## Examples

### SCSS

#### Don't

```scss options={"list":["scss"]}
@import "a.sass";
```

#### Do

```scss options={"list":["scss"]}
@import "a.scss";
```
