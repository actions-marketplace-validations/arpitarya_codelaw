---
id: js/no-async-promise-finally
title: "No async callback in finally"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the cleanup must finish first decides the rewrite"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-async-promise-finally]
since: 0.5.0
---

The callback passed to `.finally()` is not `async`.

**Why:** The chain waits for its promise, but its result is ignored and a rejection replaces the original outcome, which surprises readers.

```codelaw

match:
  any: [{kind: arrow_function}, {kind: function_expression}]
  regex: '^async\b'
  inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, has: { field: property, regex: '^finally$' } } } }
message: "An async `finally` callback delays the chain and swallows nothing it throws into the result; make the cleanup synchronous or await it before"
```

## How to fix

1. Make the callback synchronous, or move the awaited cleanup into a `try … finally` in an async function.

## Examples

### JavaScript

#### Don't

```js
load().finally(async () => {
  await close();
});
```

#### Do

```js
load().finally(() => {
  close();
});
```

### JSX

#### Don't

```jsx
load().finally(async () => {
  await close();
});
```

#### Do

```jsx
load().finally(() => {
  close();
});
```

### TypeScript

#### Don't

```ts
load().finally(async () => {
  await close();
});
```

#### Do

```ts
load().finally(() => {
  close();
});
```

### TSX

#### Don't

```tsx
load().finally(async () => {
  await close();
});
```

#### Do

```tsx
load().finally(() => {
  close();
});
```
