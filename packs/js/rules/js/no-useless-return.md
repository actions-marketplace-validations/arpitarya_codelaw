---
id: js/no-useless-return
title: "No return; at the end of a function"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-useless-return]
since: 0.5.0
---

A function does not end with a bare `return;`.

**Why:** The function returns there anyway.

```codelaw
match:
  kind: return_statement
  regex: '^return\s*;?$'
  nthChild: { position: 1, reverse: true }
  inside:
    kind: statement_block
    inside: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}], field: body }
message: "This `return` is the function's last statement and does nothing"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
function f() {
  run();
  return;
}
```

#### Do

```js
function f() {
  run();
}
```

#### Fixed

```js
function f() {
  run();
}
```

### JSX

#### Don't

```jsx
function f() {
  run();
  return;
}
```

#### Do

```jsx
function f() {
  run();
}
```

#### Fixed

```jsx
function f() {
  run();
}
```

### TypeScript

#### Don't

```ts
function f() {
  run();
  return;
}
```

#### Do

```ts
function f() {
  run();
}
```

#### Fixed

```ts
function f() {
  run();
}
```

### TSX

#### Don't

```tsx
function f() {
  run();
  return;
}
```

#### Do

```tsx
function f() {
  run();
}
```

#### Fixed

```tsx
function f() {
  run();
}
```
