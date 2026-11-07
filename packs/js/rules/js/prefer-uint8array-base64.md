---
id: js/prefer-uint8array-base64
title: "Uint8Array base64 methods rather than btoa and atob"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "only a `Uint8Array` has `toBase64()`; whether the bytes are one is not known here, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-uint8array-base64]
since: 0.5.0
---

Bytes are encoded with `bytes.toBase64()` and decoded with `Uint8Array.fromBase64(text)`, not through `btoa(String.fromCharCode(...bytes))` and `Uint8Array.from(atob(text), (c) => c.charCodeAt(0))`.

**Why:** The built-ins skip the binary string in between, and spreading a large array into `fromCharCode` can overflow the stack.

```codelaw
match:
  any:
    - pattern: "btoa(String.fromCharCode(...$B))"
      all: [{ pattern: $ENC }]
    - pattern: "Uint8Array.from(atob($T), ($C) => $C.charCodeAt(0))"
      all: [{ pattern: $DEC }]
message: "Use the Uint8Array base64 methods"
fix:
  - replace: { target: $ENC, with: "$B.toBase64()" }
  - replace: { target: $DEC, with: "Uint8Array.fromBase64($T)" }
```

## Examples

### JavaScript

#### Don't

```js
const text = btoa(String.fromCharCode(...bytes));
const back = Uint8Array.from(atob(text), (c) => c.charCodeAt(0));
```

#### Do

```js
const text = bytes.toBase64();
const back = Uint8Array.fromBase64(text);
```


### JSX

#### Don't

```jsx
const text = btoa(String.fromCharCode(...bytes));
const back = Uint8Array.from(atob(text), (c) => c.charCodeAt(0));
```

#### Do

```jsx
const text = bytes.toBase64();
const back = Uint8Array.fromBase64(text);
```


### TypeScript

#### Don't

```ts
const text = btoa(String.fromCharCode(...bytes));
const back = Uint8Array.from(atob(text), (c) => c.charCodeAt(0));
```

#### Do

```ts
const text = bytes.toBase64();
const back = Uint8Array.fromBase64(text);
```


### TSX

#### Don't

```tsx
const text = btoa(String.fromCharCode(...bytes));
const back = Uint8Array.from(atob(text), (c) => c.charCodeAt(0));
```

#### Do

```tsx
const text = bytes.toBase64();
const back = Uint8Array.fromBase64(text);
```
