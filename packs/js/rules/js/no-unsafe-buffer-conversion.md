---
id: js/no-unsafe-buffer-conversion
title: "Pass byteOffset and byteLength when viewing a buffer's .buffer"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the edit assumes the value is a `Buffer` or typed array, whose `.buffer` is a shared backing store; that is not known here"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unsafe-buffer-conversion]
since: 0.5.0
---

`.buffer` on a `Buffer` or typed array is the whole backing `ArrayBuffer`, which a pooled `Buffer` shares with others. A view over it — `new Uint8Array(bytes.buffer)`, `Buffer.from(bytes.buffer)` — or a copy of it — `bytes.buffer.slice()` — sees bytes that are not `bytes`'s. Passing `byteOffset` and `byteLength` keeps to the visible ones.

**Why:** The code works in tests with fresh buffers and leaks or garbles data with pooled ones.

```codelaw
utils:
  simple:
    any:
      - kind: identifier
      - kind: member_expression
        not: { has: { stopBy: end, any: [{ kind: call_expression }, { kind: optional_chain }] } }
match:
  any:
    - all:
        - pattern: "new $T($B.buffer)"
        - has: { field: constructor, regex: '^(Uint8Array|Int8Array|Uint8ClampedArray)$' }
        - has: { field: arguments, pattern: $A1 }
    - all:
        - pattern: "new $T($B.buffer)"
        - has: { field: constructor, regex: '^(Uint16Array|Int16Array|Uint32Array|Int32Array|Float16Array|Float32Array|Float64Array|BigInt64Array|BigUint64Array)$' }
        - has: { field: arguments, pattern: $A2 }
    - all:
        - pattern: "Buffer.from($B.buffer)"
        - has: { field: arguments, pattern: $A1 }
    - all:
        - pattern: "$B.buffer.slice()"
        - has: { field: arguments, pattern: $A3 }
constraints:
  B: { matches: simple }
message: "Keep to the view's bytes: pass `byteOffset` and `byteLength`"
fix:
  - replace: { target: $A1, with: "($B.buffer, $B.byteOffset, $B.byteLength)" }
  - replace: { target: $A2, with: "($B.buffer, $B.byteOffset, $B.byteLength / $T.BYTES_PER_ELEMENT)" }
  - replace: { target: $A3, with: "($B.byteOffset, $B.byteOffset + $B.byteLength)" }
```

## Examples

### JavaScript

#### Don't

```js
const view = new Uint8Array(chunk.buffer);
```

#### Do

```js
const view = new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength);
```


### JSX

#### Don't

```jsx
const view = new Uint8Array(chunk.buffer);
```

#### Do

```jsx
const view = new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength);
```


### TypeScript

#### Don't

```ts
const view = new Uint8Array(chunk.buffer);
```

#### Do

```ts
const view = new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength);
```


### TSX

#### Don't

```tsx
const view = new Uint8Array(chunk.buffer);
```

#### Do

```tsx
const view = new Uint8Array(chunk.buffer, chunk.byteOffset, chunk.byteLength);
```
