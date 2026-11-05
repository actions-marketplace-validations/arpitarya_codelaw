---
id: js/no-useless-promise-resolve-reject
title: "No Promise.resolve in an async function return"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-promise-resolve-reject]
since: 0.5.0
---

An `async` function returns a value, not `Promise.resolve(value)`.

**Why:** An async function already wraps its result.

```codelaw
match:
  pattern: "return Promise.resolve($X);"
  inside:
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}] }
    any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}]
    regex: '^(export\s+)?(static\s+)?async\b'
message: "Return the value; `async` wraps it"
fix: "return $X;"
```

## Examples

### JavaScript

#### Don't

```js
async function get() {
  return Promise.resolve(1);
}
```

#### Do

```js
async function get() {
  return 1;
}
```

### JSX

#### Don't

```jsx
async function get() {
  return Promise.resolve(1);
}
```

#### Do

```jsx
async function get() {
  return 1;
}
```

### TypeScript

#### Don't

```ts
async function get() {
  return Promise.resolve(1);
}
```

#### Do

```ts
async function get() {
  return 1;
}
```

### TSX

#### Don't

```tsx
async function get() {
  return Promise.resolve(1);
}
```

#### Do

```tsx
async function get() {
  return 1;
}
```
