---
id: react/a11y-html-has-lang
title: "The html element has a lang"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the page's language is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-jsx-a11y/html-has-lang]
since: 0.5.0
---

An `<html>` element carries a `lang` attribute.

**Why:** Screen readers pick their pronunciation from `lang`; without it they guess, often wrongly.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^html$}
  - not:
      has: {kind: jsx_attribute, regex: ^lang\b}
  - not:
      has: {kind: jsx_expression}
message: Add a `lang` attribute to `<html>`
```

## How to fix

1. Add `lang` with the page's language, for example `lang="en"`.

## Examples

### JavaScript

#### Don't

```js
const page = <html><body /></html>;
```

#### Do

```js
const page = <html lang="en"><body /></html>;
```

### JSX

#### Don't

```jsx
const page = <html><body /></html>;
```

#### Do

```jsx
const page = <html lang="en"><body /></html>;
```

### TSX

#### Don't

```tsx
const page = <html><body /></html>;
```

#### Do

```tsx
const page = <html lang="en"><body /></html>;
```
