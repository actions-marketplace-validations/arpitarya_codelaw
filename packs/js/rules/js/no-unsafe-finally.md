---
id: js/no-unsafe-finally
title: "No return, throw, break or continue in finally"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the control statement lets the try or catch result through, and a value it carried is dropped; the reviewer confirms"
stability: preview
ported_from: [eslint/no-unsafe-finally]
since: 0.1.0
---

A `finally` block doesn't end the function or loop with `return`, `throw`, `break` or `continue`.

**Why:** Control flow in `finally` silently replaces whatever the `try` or `catch` block returned or
threw.

```codelaw
match:
  any:
    - kind: return_statement
    - kind: throw_statement
    - kind: break_statement
    - kind: continue_statement
  inside:
    kind: finally_clause
    stopBy:
      any: [{kind: function_declaration}, {kind: function_expression}, {kind: arrow_function}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}, { kind: class_declaration }]
  not:
    all:
      - any: [{ kind: break_statement }, { kind: continue_statement }]
      - inside:
          stopBy: { kind: finally_clause }
          any: [{ kind: for_statement }, { kind: for_in_statement }, { kind: while_statement }, { kind: do_statement }, { kind: switch_statement }]
message: "Control flow in `finally` overrides the try block's return or exception"
fix:
  - remove
```

## How to fix

1. Check the statement removed was not carrying a value the function needs; if it was, move it into the `try` or `catch` block.

## Examples

### JavaScript

#### Don't

```js
function load() {
  try {
    return fetchAll();
  } finally {
    return null;
  }
}
```

#### Fixed

```js
function load() {
  try {
    return fetchAll();
  } finally {
  }
}
```

#### Do

```js
function load() {
  try {
    return fetchAll();
  } finally {
    cleanup();
  }
}
```

### JSX

#### Don't

```jsx
function load() {
  try {
    return fetchAll();
  } finally {
    return null;
  }
}
```

#### Fixed

```jsx
function load() {
  try {
    return fetchAll();
  } finally {
  }
}
```

#### Do

```jsx
function load() {
  try {
    return fetchAll();
  } finally {
    cleanup();
  }
}
```

### TypeScript

#### Don't

```ts
function load() {
  try {
    return fetchAll();
  } finally {
    return null;
  }
}
```

#### Fixed

```ts
function load() {
  try {
    return fetchAll();
  } finally {
  }
}
```

#### Do

```ts
function load() {
  try {
    return fetchAll();
  } finally {
    cleanup();
  }
}
```

### TSX

#### Don't

```tsx
function load() {
  try {
    return fetchAll();
  } finally {
    return null;
  }
}
```

#### Fixed

```tsx
function load() {
  try {
    return fetchAll();
  } finally {
  }
}
```

#### Do

```tsx
function load() {
  try {
    return fetchAll();
  } finally {
    cleanup();
  }
}
```
