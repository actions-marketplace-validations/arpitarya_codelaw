---
id: js/no-extend-native
title: "No changes to built-in prototypes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the method becomes a standalone function, which changes every call site"
stability: preview
ported_from: [eslint/no-extend-native]
since: 0.5.0
---

The prototypes of built-in objects (`Object`, `Array`, `String`, …) are not extended or changed.

**Why:** Every other script on the page sees the change, and a later standard method of the same name breaks it.

```codelaw
match:
  any:
    - pattern: $C.prototype.$P = $V
    - pattern: "$C.prototype[$K] = $V"
    - pattern: Object.defineProperty($C.prototype, $$$A)
    - pattern: Object.defineProperties($C.prototype, $$$A)
constraints:
  C: { regex: '^(Object|Array|String|Number|Boolean|Function|Symbol|BigInt|Date|RegExp|Error|Promise|Map|Set|WeakMap|WeakSet|ArrayBuffer|DataView|JSON|Math|Reflect|Proxy|Intl|(Int|Uint|Float)(8|16|32|64)(Clamped)?Array|BigInt64Array|BigUint64Array)$' }
message: "Do not modify `$C.prototype`"
```

## How to fix

1. Write a standalone function that takes the value as its argument.

## Examples

### JavaScript

#### Don't

```js
Array.prototype.last = function () { return this[this.length - 1]; };
```

#### Do

```js
const last = (xs) => xs[xs.length - 1];
```

### JSX

#### Don't

```jsx
Array.prototype.last = function () { return this[this.length - 1]; };
```

#### Do

```jsx
const last = (xs) => xs[xs.length - 1];
```

### TypeScript

#### Don't

```ts
Array.prototype.last = function () { return this[this.length - 1]; };
```

#### Do

```ts
const last = (xs) => xs[xs.length - 1];
```

### TSX

#### Don't

```tsx
Array.prototype.last = function () { return this[this.length - 1]; };
```

#### Do

```tsx
const last = (xs) => xs[xs.length - 1];
```
