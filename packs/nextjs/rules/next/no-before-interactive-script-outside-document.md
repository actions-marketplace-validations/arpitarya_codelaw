---
id: next/no-before-interactive-script-outside-document
title: "beforeInteractive scripts live in the document"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "moving the script to the document is a structural change"
fix_exception: restructure
exclude: ["**/pages/_document.*", "**/app/layout.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-before-interactive-script-outside-document"]
since: 0.5.0
---

A `<Script strategy="beforeInteractive">` is placed only in `pages/_document` or the root `app` layout.

**Why:** Elsewhere the strategy is not honoured and the script loads as an ordinary one.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^Script$}
  - has: {kind: jsx_attribute, regex: '^strategy=["'']beforeInteractive["'']'}
message: Move this `beforeInteractive` script to `pages/_document` or the root layout
```

## How to fix

1. Move the `<Script>` into `pages/_document` or `app/layout`.

## Examples

### JavaScript

#### Don't

```js
const s = <Script src="/x.js" strategy="beforeInteractive" />;
```

#### Do

```js
const s = <Script src="/x.js" strategy="afterInteractive" />;
```

### JSX

#### Don't

```jsx
const s = <Script src="/x.js" strategy="beforeInteractive" />;
```

#### Do

```jsx
const s = <Script src="/x.js" strategy="afterInteractive" />;
```

### TSX

#### Don't

```tsx
const s = <Script src="/x.js" strategy="beforeInteractive" />;
```

#### Do

```tsx
const s = <Script src="/x.js" strategy="afterInteractive" />;
```
