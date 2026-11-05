---
id: next/no-unwanted-polyfillio
title: "No polyfill.io"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing the script drops its polyfills; the reviewer confirms Next.js already covers the features it served (offered for a self-closing script written as a child element)"
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
  - any:
    - kind: jsx_self_closing_element
      pattern: $S
      inside: {kind: jsx_element}
    - any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
message: Do not load scripts from polyfill.io
fix:
  - remove: $S
```

## How to fix

1. Remove the script; add a specific polyfill package only for a feature Next.js does not already cover.

## Examples

### JavaScript

#### Don't

```js
const s = <div><script src="https://cdn.polyfill.io/v3/polyfill.min.js" /></div>;
```

#### Do

```js
const s = <div></div>;
```

### JSX

#### Don't

```jsx
const s = <div><script src="https://cdn.polyfill.io/v3/polyfill.min.js" /></div>;
```

#### Do

```jsx
const s = <div></div>;
```

### TSX

#### Don't

```tsx
const s = <div><script src="https://cdn.polyfill.io/v3/polyfill.min.js" /></div>;
```

#### Do

```tsx
const s = <div></div>;
```

## Differences from the source

Every polyfill.io script is reported; the source reports only the features Next.js already polyfills.
