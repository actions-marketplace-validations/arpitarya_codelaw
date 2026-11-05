---
id: next/no-img-element
title: "next/image, not img"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "`<Image>` needs a width and height or `fill`, which the author supplies"
fix_exception: value
stability: preview
ported_from: ["@next/eslint-plugin-next/no-img-element"]
since: 0.5.0
---

Images use `<Image>` from `next/image`, not `<img>` (an `<img>` inside `<picture>` is allowed).

**Why:** `<Image>` resizes, lazy-loads and serves modern formats; a raw `<img>` ships the original file to every device.

```codelaw
match:
  any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
  has: { field: name, regex: '^img$' }
  not: { inside: { kind: jsx_element, has: { field: open_tag, has: { field: name, regex: '^picture$' } } } }
message: "Use `<Image>` from `next/image` instead of `<img>`"
```

## How to fix

1. `import Image from 'next/image'` and replace `<img>` with `<Image>`, giving it `width` and `height` or `fill`.

## Examples

### JavaScript

#### Don't

```js
const i = <img src="/logo.png" alt="Logo" />;
```

#### Do

```js
const i = <Image src="/logo.png" alt="Logo" width={64} height={64} />;
```

### JSX

#### Don't

```jsx
const i = <img src="/logo.png" alt="Logo" />;
```

#### Do

```jsx
const i = <Image src="/logo.png" alt="Logo" width={64} height={64} />;
```

### TSX

#### Don't

```tsx
const i = <img src="/logo.png" alt="Logo" />;
```

#### Do

```tsx
const i = <Image src="/logo.png" alt="Logo" width={64} height={64} />;
```
