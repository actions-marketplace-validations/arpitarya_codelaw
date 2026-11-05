---
id: next/no-script-component-in-head
title: "No next/script inside next/head"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the script moves to the page body"
fix_pending: "suggest"
stability: preview
ported_from: ["@next/eslint-plugin-next/no-script-component-in-head"]
since: 0.5.0
---

`<Script>` is not placed inside `<Head>`.

**Why:** `next/script` manages where the script goes; inside `next/head` it is not loaded as intended.

```codelaw
match:
  any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
  has: { field: name, regex: '^Script$' }
  inside:
    stopBy: end
    kind: jsx_element
    has: { field: open_tag, has: { field: name, regex: '^Head$' } }
message: "Move `<Script>` out of `<Head>`"
```

## How to fix

1. Move the `<Script>` into the page body.

## Examples

### JavaScript

#### Don't

```js
const p = <Head><Script src="/a.js" /></Head>;
```

#### Do

```js
const p = <><Head /><Script src="/a.js" /></>;
```

### JSX

#### Don't

```jsx
const p = <Head><Script src="/a.js" /></Head>;
```

#### Do

```jsx
const p = <><Head /><Script src="/a.js" /></>;
```

### TSX

#### Don't

```tsx
const p = <Head><Script src="/a.js" /></Head>;
```

#### Do

```tsx
const p = <><Head /><Script src="/a.js" /></>;
```
