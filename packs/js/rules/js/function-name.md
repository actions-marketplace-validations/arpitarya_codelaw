---
id: js/function-name
title: "Function names in camelCase"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's choice and changes every call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/function-name]
since: 0.5.0
---

A function declaration's name is camelCase (PascalCase is allowed for components and constructors).

**Why:** One naming style makes code read the same everywhere.

```codelaw

match:
  kind: function_declaration
  has: { field: name, not: { regex: '^[_a-z][a-zA-Z0-9]*$|^[A-Z][a-zA-Z0-9]*$' } }
message: "Name the function in camelCase"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
function load_user() {}
```

#### Do

```js
function loadUser() {}
```

### JSX

#### Don't

```jsx
function load_user() {}
```

#### Do

```jsx
function loadUser() {}
```

### TypeScript

#### Don't

```ts
function load_user() {}
```

#### Do

```ts
function loadUser() {}
```

### TSX

#### Don't

```tsx
function load_user() {}
```

#### Do

```tsx
function loadUser() {}
```
