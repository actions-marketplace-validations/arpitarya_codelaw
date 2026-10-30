---
id: next/no-styled-jsx-in-document
title: "No styled-jsx in _document"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the styles move to a global stylesheet or `_app`"
files: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-styled-jsx-in-document"]
since: 0.5.0
---

`pages/_document` does not use `<style jsx>`.

**Why:** The document renders only on the server, so styled-jsx styles there are never applied.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^style$}
  - has: {kind: jsx_attribute, regex: ^jsx\b}
message: Do not use `<style jsx>` in `_document`
```

## How to fix

1. Move the styles to a global CSS file imported in `_app`.

## Examples

### JavaScript

#### Don't

```js path=pages/_document.js
const d = <Html><style jsx>{`body { margin: 0; }`}</style></Html>;
```

#### Do

```js path=pages/_document.js
const d = <Html><Head /></Html>;
```

### JSX

#### Don't

```jsx path=pages/_document.jsx
const d = <Html><style jsx>{`body { margin: 0; }`}</style></Html>;
```

#### Do

```jsx path=pages/_document.jsx
const d = <Html><Head /></Html>;
```

### TSX

#### Don't

```tsx path=pages/_document.tsx
const d = <Html><style jsx>{`body { margin: 0; }`}</style></Html>;
```

#### Do

```tsx path=pages/_document.tsx
const d = <Html><Head /></Html>;
```
