---
id: js/prefer-top-level-await
title: "Top-level await rather than an async IIFE"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the file must be a module, and a rejection now fails the import"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-top-level-await]
since: 0.5.0
---

A module does not wrap its top-level code in an async immediately-invoked function.

**Why:** Top-level `await` runs the same code without the wrapper, and a rejection is not lost.

```codelaw

match:
  kind: expression_statement
  inside: { kind: program }
  has: { kind: call_expression, has: { field: function, kind: parenthesized_expression, has: { kind: arrow_function, regex: '^async\b' } } }
message: "Use top-level `await` in a module instead of an async IIFE"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
(async () => {
  await start();
})();
```

#### Do

```js
await start();
```

### JSX

#### Don't

```jsx
(async () => {
  await start();
})();
```

#### Do

```jsx
await start();
```

### TypeScript

#### Don't

```ts
(async () => {
  await start();
})();
```

#### Do

```ts
await start();
```

### TSX

#### Don't

```tsx
(async () => {
  await start();
})();
```

#### Do

```tsx
await start();
```
