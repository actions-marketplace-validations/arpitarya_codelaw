---
id: js/no-await-in-loop
title: "No await inside a loop"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Promise.all` runs the iterations together, which changes ordering and failure handling; the reviewer confirms they are independent"
stability: preview
ported_from: [eslint/no-await-in-loop]
since: 0.5.0
---

`await` is not used directly inside a loop body.

**Why:** Each iteration waits for the last; independent work runs far faster with `Promise.all`.

```codelaw
match:
  kind: await_expression
  any:
    - inside:
        stopBy: end
        all:
          - { pattern: $LOOP }
          - pattern: "for (const $X of $XS) { await $C; }"
          - not: { regex: '^for\s+await' }
    - inside:
        stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] }
        any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}]
        not: { regex: '^for\s+await' }
message: "`await` in a loop runs each step one after another; use `Promise.all` if they are independent"
fix:
  - replace: { target: $LOOP, with: "await Promise.all($XS.map(($X) => $C));" }
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

#### Fixed

```js
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
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

#### Fixed

```jsx
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
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

#### Fixed

```ts
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
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

#### Fixed

```tsx
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```

#### Do

```tsx
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```
