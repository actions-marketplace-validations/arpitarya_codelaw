---
id: next/no-duplicate-head
title: "One Head in _document"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the two heads' contents have to be merged by hand"
fix_exception: value
files: ["**/pages/_document.*"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-duplicate-head"]
since: 0.5.0
---

`pages/_document` renders `<Head>` once.

**Why:** A second `<Head>` duplicates tags and Next.js warns at run time.

```codelaw
match:
  any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
  has: { field: name, regex: '^Head$' }
  inside:
    kind: jsx_element
    follows:
      stopBy: end
      any:
        - { kind: jsx_self_closing_element, has: { field: name, regex: '^Head$' } }
        - { kind: jsx_element, has: { field: open_tag, has: { field: name, regex: '^Head$' } } }
message: "Render `<Head>` once in `_document`"
```

## How to fix

1. Merge the two `<Head>` elements into one.

## Examples

### JavaScript

#### Don't

```js path=pages/_document.js
const d = (
  <Html>
    <Head />
    <Head></Head>
  </Html>
);
```

#### Do

```js path=pages/_document.js
const d = (
  <Html>
    <Head />
  </Html>
);
```

### JSX

#### Don't

```jsx path=pages/_document.jsx
const d = (
  <Html>
    <Head />
    <Head></Head>
  </Html>
);
```

#### Do

```jsx path=pages/_document.jsx
const d = (
  <Html>
    <Head />
  </Html>
);
```

### TSX

#### Don't

```tsx path=pages/_document.tsx
const d = (
  <Html>
    <Head />
    <Head></Head>
  </Html>
);
```

#### Do

```tsx path=pages/_document.tsx
const d = (
  <Html>
    <Head />
  </Html>
);
```
