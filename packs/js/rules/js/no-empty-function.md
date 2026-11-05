---
id: js/no-empty-function
title: "No empty functions"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the function's code, or the comment explaining why it is empty, is the author's"
fix_exception: authored
stability: preview
ported_from: [eslint/no-empty-function]
since: 0.5.0
---

A function has a body, or a comment saying why it is empty.

**Why:** An empty function is either unfinished or deliberate, and a reader cannot tell which.

```codelaw
match:
  kind: statement_block
  regex: '^\{\s*\}$'
  inside: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}], field: body }
message: "This function is empty; add its code or a comment explaining why"
```

## How to fix

1. Write the function's code, or a comment saying why it does nothing.

## Examples

### JavaScript

#### Don't

```js
function noop() {}
```

#### Do

```js
function noop() {
  // Called by the plugin API, which requires a handler.
}
```

### JSX

#### Don't

```jsx
function noop() {}
```

#### Do

```jsx
function noop() {
  // Called by the plugin API, which requires a handler.
}
```

### TypeScript

#### Don't

```ts
function noop() {}
```

#### Do

```ts
function noop() {
  // Called by the plugin API, which requires a handler.
}
```

### TSX

#### Don't

```tsx
function noop() {}
```

#### Do

```tsx
function noop() {
  // Called by the plugin API, which requires a handler.
}
```
