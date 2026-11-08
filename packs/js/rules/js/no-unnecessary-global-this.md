---
id: js/no-unnecessary-global-this
title: "A known global read directly, not off globalThis"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a local binding with the global's name would take the read once `globalThis.` is dropped, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-global-this]
since: 0.5.0
---

`globalThis.JSON`, `globalThis.Array` and the other built-in globals are read by their name. Writes, optional reads and existence checks (`if (globalThis.x)`, `typeof`) are left, since they ask whether the global is there.

**Why:** The prefix adds nothing when the global always exists.

```codelaw
match:
  kind: member_expression
  pattern: "globalThis.$G"
  has: { field: property, regex: '^(Array|ArrayBuffer|BigInt|Boolean|DataView|Date|Error|EvalError|Float32Array|Float64Array|Function|Infinity|Int8Array|Int16Array|Int32Array|Intl|JSON|Map|Math|NaN|Number|Object|Promise|Proxy|RangeError|ReferenceError|Reflect|RegExp|Set|String|Symbol|SyntaxError|TypeError|URIError|Uint8Array|Uint8ClampedArray|Uint16Array|Uint32Array|WeakMap|WeakSet|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|isFinite|isNaN|parseFloat|parseInt)$' }
  not:
    any:
      - inside: { kind: assignment_expression, field: left }
      - inside: { kind: augmented_assignment_expression, field: left }
      - inside: { kind: unary_expression, regex: '^(typeof|delete)\b' }
      - inside: { kind: parenthesized_expression, inside: { kind: if_statement } }
      - has: { kind: optional_chain }
message: "Read `$G` directly"
fix: "$G"
```

## Examples

### JavaScript

#### Don't

```js
const copy = globalThis.Array.from(items);
```

#### Do

```js
const copy = Array.from(items);
```


### JSX

#### Don't

```jsx
const copy = globalThis.Array.from(items);
```

#### Do

```jsx
const copy = Array.from(items);
```


### TypeScript

#### Don't

```ts
const copy = globalThis.Array.from(items);
```

#### Do

```ts
const copy = Array.from(items);
```


### TSX

#### Don't

```tsx
const copy = globalThis.Array.from(items);
```

#### Do

```tsx
const copy = Array.from(items);
```
