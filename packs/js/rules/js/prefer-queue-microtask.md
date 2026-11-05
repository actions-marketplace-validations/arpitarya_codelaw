---
id: js/prefer-queue-microtask
title: "queueMicrotask rather than Promise.resolve().then"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "an error thrown by the callback is reported instead of rejecting a promise nobody reads"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-queue-microtask]
since: 0.5.0
---

`Promise.resolve().then(fn)` used to schedule work is written `queueMicrotask(fn)`.

**Why:** `queueMicrotask` says what it does and does not create a promise.

```codelaw
match:
  pattern: "Promise.resolve().then($F)"
  inside: { kind: expression_statement }
message: "Use `queueMicrotask()`"
fix: "queueMicrotask($F)"
```

## Examples

### JavaScript

#### Don't

```js
Promise.resolve().then(flush);
```

#### Do

```js
queueMicrotask(flush);
```

### JSX

#### Don't

```jsx
Promise.resolve().then(flush);
```

#### Do

```jsx
queueMicrotask(flush);
```

### TypeScript

#### Don't

```ts
Promise.resolve().then(flush);
```

#### Do

```ts
queueMicrotask(flush);
```

### TSX

#### Don't

```tsx
Promise.resolve().then(flush);
```

#### Do

```tsx
queueMicrotask(flush);
```
