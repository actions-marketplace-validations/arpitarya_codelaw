---
id: next/no-unwanted-polyfillio
title: "No polyfill.io"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "what to keep depends on which features the script was polyfilling"
stability: preview
ported_from: ["@next/eslint-plugin-next/no-unwanted-polyfillio"]
since: 0.5.0
---

No script is loaded from `polyfill.io` (or `cdn.polyfill.io`).

**Why:** Next.js already includes the polyfills browsers need, and the polyfill.io domain was taken over in 2024 and served malicious code.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^(script|Script)$}
  - has: {kind: jsx_attribute, regex: ^src=.*polyfill\.io}
message: Do not load scripts from polyfill.io
```

## How to fix

1. Remove the script; add a specific polyfill package only for a feature Next.js does not already cover.

## Examples

### JavaScript

#### Don't

```js
const s = <script src="https://cdn.polyfill.io/v3/polyfill.min.js" />;
```

#### Do

```js
const s = null;
```

### JSX

#### Don't

```jsx
const s = <script src="https://cdn.polyfill.io/v3/polyfill.min.js" />;
```

#### Do

```jsx
const s = null;
```

### TSX

#### Don't

```tsx
const s = <script src="https://cdn.polyfill.io/v3/polyfill.min.js" />;
```

#### Do

```tsx
const s = null;
```

## Differences from the source

Every polyfill.io script is reported; the source reports only the features Next.js already polyfills.
