---
id: react/a11y-lang
title: "lang is a valid language tag"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the page's language is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/lang]
since: 0.5.0
---

The `lang` of `<html>`, written as a string, is a well-formed BCP 47 language tag.

**Why:** An invalid tag (`lang="english"`) is ignored, and screen readers fall back to guessing the language.

```codelaw
match:
  kind: jsx_attribute
  regex: '^lang=["'']'
  not: { regex: '^lang=(["''])[a-zA-Z]{2,3}(-[a-zA-Z0-9]{2,8})*["'']$' }
  inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], has: { field: name, regex: '^html$' } }
message: "`$$MATCH` is not a valid language tag"
```

## How to fix

1. Use a BCP 47 tag such as `en`, `en-GB` or `pt-BR`.

## Examples

### JavaScript

#### Don't

```js
const page = <html lang="english"><body /></html>;
```

#### Do

```js
const page = <html lang="en"><body /></html>;
```

### JSX

#### Don't

```jsx
const page = <html lang="english"><body /></html>;
```

#### Do

```jsx
const page = <html lang="en"><body /></html>;
```

### TSX

#### Don't

```tsx
const page = <html lang="english"><body /></html>;
```

#### Do

```tsx
const page = <html lang="en"><body /></html>;
```
