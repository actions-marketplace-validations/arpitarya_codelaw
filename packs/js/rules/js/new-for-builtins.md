---
id: js/new-for-builtins
title: "new for constructors, no new for Symbol and BigInt"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Date()` returns a string and `new Date()` an object, so adding `new` can change the value"
stability: preview
ported_from: [eslint-plugin-unicorn/new-for-builtins]
since: 0.5.0
---

Built-in constructors are called with `new` (`new Map()`, `new Date()`); `Symbol` and `BigInt` are already covered by `js/no-new-native-nonconstructor`. `String`, `Number` and `Boolean` called as functions convert, so they are left alone.

**Why:** Some constructors work without `new` and some throw; `new` everywhere is one rule to remember. `Date()` without `new` even returns a string.

```codelaw

match:
  kind: call_expression
  has: { field: function, kind: identifier, regex: '^(Object|Array|ArrayBuffer|BigInt64Array|BigUint64Array|DataView|Date|Error|AggregateError|EvalError|RangeError|ReferenceError|SyntaxError|TypeError|URIError|Float32Array|Float64Array|Function|Int8Array|Int16Array|Int32Array|Map|WeakMap|Set|WeakSet|Promise|RegExp|Uint8Array|Uint16Array|Uint32Array|Uint8ClampedArray|SharedArrayBuffer|Proxy|WeakRef|FinalizationRegistry)$', pattern: $C }
where:
  $C: builtin
message: "Call `$C` with `new`"
fix:
  - insert-before: "new "
```

## Examples

### JavaScript

#### Don't

```js
const seen = Map();
```

#### Do

```js
const seen = new Map();
```

### JSX

#### Don't

```jsx
const seen = Map();
```

#### Do

```jsx
const seen = new Map();
```

### TypeScript

#### Don't

```ts
const seen = Map();
```

#### Do

```ts
const seen = new Map();
```

### TSX

#### Don't

```tsx
const seen = Map();
```

#### Do

```tsx
const seen = new Map();
```
