---
id: js/prefer-uint8array-hex
title: "Uint8Array toHex() rather than mapping bytes to hex"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "only a `Uint8Array` has `toHex()`; whether the bytes are one is not known here, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-uint8array-hex]
since: 0.5.0
---

Bytes are written as hex with `bytes.toHex()`, not by mapping each byte through `toString(16).padStart(2, "0")` and joining.

**Why:** The built-in is shorter and does not build a string per byte.

```codelaw
match:
  any:
    - pattern: "Array.from($B, ($X) => $X.toString(16).padStart(2, $Z)).join($E)"
    - pattern: "[...$B].map(($X) => $X.toString(16).padStart(2, $Z)).join($E)"
constraints:
  Z: { regex: '^["'']0["'']$' }
  E: { regex: '^(""|'''')$' }
message: "Use `$B.toHex()`"
fix: "$B.toHex()"
```

## Examples

### JavaScript

#### Don't

```js
const hex = Array.from(digest, (b) => b.toString(16).padStart(2, "0")).join("");
```

#### Do

```js
const hex = digest.toHex();
```


### JSX

#### Don't

```jsx
const hex = Array.from(digest, (b) => b.toString(16).padStart(2, "0")).join("");
```

#### Do

```jsx
const hex = digest.toHex();
```


### TypeScript

#### Don't

```ts
const hex = Array.from(digest, (b) => b.toString(16).padStart(2, "0")).join("");
```

#### Do

```ts
const hex = digest.toHex();
```


### TSX

#### Don't

```tsx
const hex = Array.from(digest, (b) => b.toString(16).padStart(2, "0")).join("");
```

#### Do

```tsx
const hex = digest.toHex();
```
