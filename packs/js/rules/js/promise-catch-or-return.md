---
id: js/promise-catch-or-return
title: "A promise chain is returned, awaited or ends in catch"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "returning or awaiting the chain changes the function's result and timing, and a `.catch()` is a decision about the error; the reviewer picks"
stability: preview
ported_from: [eslint-plugin-promise/catch-or-return]
since: 0.5.0
---

A promise chain used as a statement on its own ends with `.catch()`; otherwise it is returned or awaited so its caller handles a rejection.

**Why:** A rejection in a chain nobody returns, awaits or catches becomes an unhandled rejection — in Node, a crash.

```codelaw
match:
  kind: expression_statement
  all:
    - any:
        - pattern: $RET
          inside: { stopBy: end, any: [{ kind: function_declaration }, { kind: function_expression }, { kind: arrow_function }, { kind: method_definition }, { kind: generator_function_declaration }, { kind: generator_function }] }
        - kind: expression_statement
    - any:
        - pattern: $AW
          inside: { stopBy: { any: [{ kind: function_declaration }, { kind: function_expression }, { kind: arrow_function }, { kind: method_definition }, { kind: generator_function_declaration }, { kind: generator_function }] }, any: [{ kind: function_declaration }, { kind: function_expression }, { kind: arrow_function }, { kind: method_definition }, { kind: generator_function_declaration }, { kind: generator_function }], regex: '^(export\s+)?(default\s+)?(static\s+)?async\b' }
        - kind: expression_statement
  has:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      any:
        - has: { field: property, regex: '^then$' }
        - all:
            - has: { field: property, regex: '^finally$' }
            - not: { regex: '\.catch\s*\(' }
message: "Handle this promise chain's rejection: end it with `.catch()`, or return or await it"
fix:
  choices:
    - label: return the chain
      fix:
        - insert-before: { target: $RET, text: "return " }
    - label: await the chain
      fix:
        - insert-before: { target: $AW, text: "await " }
```

## How to fix

1. Add a `.catch()` at the end of the chain that handles or reports the error, or `return` / `await` the chain so the caller does; the fix offers the last two.

## Examples

### JavaScript

#### Don't

```js
async function run(doc) {
  save(doc).then(() => notify());
}
```

#### Fixed (return the chain)

```js
async function run(doc) {
  return save(doc).then(() => notify());
}
```

#### Fixed (await the chain)

```js
async function run(doc) {
  await save(doc).then(() => notify());
}
```

#### Do

```js
async function run(doc) {
  save(doc)
    .then(() => notify())
    .catch((err) => report(err));
}
```

### JSX

#### Don't

```jsx
async function run(doc) {
  save(doc).then(() => notify());
}
```

#### Fixed (return the chain)

```jsx
async function run(doc) {
  return save(doc).then(() => notify());
}
```

#### Fixed (await the chain)

```jsx
async function run(doc) {
  await save(doc).then(() => notify());
}
```

#### Do

```jsx
async function run(doc) {
  save(doc)
    .then(() => notify())
    .catch((err) => report(err));
}
```

### TypeScript

#### Don't

```ts
async function run(doc) {
  save(doc).then(() => notify());
}
```

#### Fixed (return the chain)

```ts
async function run(doc) {
  return save(doc).then(() => notify());
}
```

#### Fixed (await the chain)

```ts
async function run(doc) {
  await save(doc).then(() => notify());
}
```

#### Do

```ts
async function run(doc) {
  save(doc)
    .then(() => notify())
    .catch((err) => report(err));
}
```

### TSX

#### Don't

```tsx
async function run(doc) {
  save(doc).then(() => notify());
}
```

#### Fixed (return the chain)

```tsx
async function run(doc) {
  return save(doc).then(() => notify());
}
```

#### Fixed (await the chain)

```tsx
async function run(doc) {
  await save(doc).then(() => notify());
}
```

#### Do

```tsx
async function run(doc) {
  save(doc)
    .then(() => notify())
    .catch((err) => report(err));
}
```
