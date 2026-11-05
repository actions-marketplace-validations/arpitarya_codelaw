---
id: js/promise-no-multiple-resolved
title: "A promise executor settles once on each path"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the first or the second call is the right one is the author's call"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint-plugin-promise/no-multiple-resolved]
since: 0.5.0
---

A promise executor does not call `resolve` or `reject` twice in a row in the same block.

**Why:** Only the first call settles the promise; the second is silently ignored, which is almost always a missing `return`.

```codelaw
utils:
  settle:
    kind: expression_statement
    has:
      kind: call_expression
      has: { field: function, kind: identifier, regex: '^(resolve|reject)$' }
match:
  matches: settle
  follows: { matches: settle, stopBy: end }
  inside:
    stopBy: end
    any: [{kind: arrow_function}, {kind: function_expression}]
    inside:
      kind: arguments
      inside:
        kind: new_expression
        has: { field: constructor, regex: '^Promise$' }
message: "The promise is already settled by an earlier call in this block; this one is ignored"
```

## How to fix

1. Add the missing `return` after the first call, or delete the call that should not run.

## Examples

### JavaScript

#### Don't

```js
new Promise((resolve, reject) => {
  if (bad) reject(err);
  resolve(value);
  reject(late);
});
```

#### Do

```js
new Promise((resolve, reject) => {
  if (bad) return reject(err);
  resolve(value);
});
```

### JSX

#### Don't

```jsx
new Promise((resolve, reject) => {
  if (bad) reject(err);
  resolve(value);
  reject(late);
});
```

#### Do

```jsx
new Promise((resolve, reject) => {
  if (bad) return reject(err);
  resolve(value);
});
```

### TypeScript

#### Don't

```ts
new Promise((resolve, reject) => {
  if (bad) reject(err);
  resolve(value);
  reject(late);
});
```

#### Do

```ts
new Promise((resolve, reject) => {
  if (bad) return reject(err);
  resolve(value);
});
```

### TSX

#### Don't

```tsx
new Promise((resolve, reject) => {
  if (bad) reject(err);
  resolve(value);
  reject(late);
});
```

#### Do

```tsx
new Promise((resolve, reject) => {
  if (bad) return reject(err);
  resolve(value);
});
```

## Differences from the source

The source follows each code path through branches; this rule reports two settling calls written one after the other in the same block.
