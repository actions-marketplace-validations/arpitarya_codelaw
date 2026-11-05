---
id: scss/at-import-partial-extension-disallowed-list
title: "No `@import` extension the project disallows"
languages: [scss]
fix: manual
fix_reason: "which extension the project uses is its own convention"
fix_exception: value
stability: preview
options:
  list: { type: "string[]", default: [], description: "the extensions disallowed" }
ported_from: [stylelint-scss/at-import-partial-extension-disallowed-list]
since: 0.5.0
---

An `@import`'s file extension is not in `list`. With no option set, the rule does nothing.

**Why:** A project loads its partials one way.

```codelaw
native: scss/at-import-partial-extension-disallowed-list
```

## How to fix

1. Drop the extension.

## Examples

### SCSS

#### Don't

```scss options={"list":["scss"]}
@import "a.scss";
```

#### Do

```scss options={"list":["scss"]}
@import "a";
```
