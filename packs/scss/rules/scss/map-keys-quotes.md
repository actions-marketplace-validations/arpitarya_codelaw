---
id: scss/map-keys-quotes
title: "Map keys are quoted"
languages: [scss]
fix: suggest
fix_reason: "quoting a key that reads as a colour or a number changes its type"
stability: preview
ported_from: [stylelint-scss/map-keys-quotes]
since: 0.5.0
---

A map's string keys are quoted: `("small": 1px)`.

**Why:** An unquoted key that happens to be a colour name is a colour.

```codelaw
native: scss/map-keys-quotes
```

## How to fix

1. Quote the key.

## Examples

### SCSS

#### Don't

```scss
$sizes: (small: 1px);
```

#### Do

```scss
$sizes: ("small": 1px);
```
