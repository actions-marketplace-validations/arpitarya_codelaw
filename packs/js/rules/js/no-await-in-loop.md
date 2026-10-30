---
id: js/no-await-in-loop
title: "No await inside a loop"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the iterations may depend on each other; only the author knows if they can run together"
stability: preview
ported_from: [eslint/no-await-in-loop]
since: 0.5.0
---

`await` is not used directly inside a loop body.

**Why:** Each iteration waits for the last; independent work runs far faster with `Promise.all`.

```codelaw
match:
  kind: await_expression
  inside:
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] }
    any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}]
    not: { regex: '^for\s+await' }
message: "`await` in a loop runs each step one after another; use `Promise.all` if they are independent"
```

## How to fix

1. Collect the promises and `await Promise.all(...)` after the loop, if the iterations are independent.

## Examples

### JavaScript

#### Don't

```js
async function f(ids) {
  for (const id of ids) {
    await load(id);
  }
}
```

#### Do

```js
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```

### JSX

#### Don't

```jsx
async function f(ids) {
  for (const id of ids) {
    await load(id);
  }
}
```

#### Do

```jsx
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```

### TypeScript

#### Don't

```ts
async function f(ids) {
  for (const id of ids) {
    await load(id);
  }
}
```

#### Do

```ts
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```

### TSX

#### Don't

```tsx
async function f(ids) {
  for (const id of ids) {
    await load(id);
  }
}
```

#### Do

```tsx
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```
