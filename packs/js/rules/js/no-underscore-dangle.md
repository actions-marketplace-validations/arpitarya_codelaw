---
id: js/no-underscore-dangle
title: "No leading or trailing underscore in names"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's choice and may collide"
stability: preview
ported_from: [eslint/no-underscore-dangle]
since: 0.5.0
---

A declared variable or function name does not start or end with `_` (`_`, `__dirname` and the like are left alone). Upstream also checks member names; this port checks declarations.

**Why:** The underscore suggests privacy that JavaScript does not enforce; `#private` fields and module scope do.

```codelaw

match:
  kind: identifier
  regex: '^_[A-Za-z0-9$]|[A-Za-z0-9$]_$'
  any:
    - inside: { kind: variable_declarator, field: name }
    - inside: { kind: function_declaration, field: name }
message: "Drop the dangling underscore from `$$MATCH`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const _cache = new Map();
```

#### Do

```js
const cache = new Map();
```

### JSX

#### Don't

```jsx
const _cache = new Map();
```

#### Do

```jsx
const cache = new Map();
```

### TypeScript

#### Don't

```ts
const _cache = new Map();
```

#### Do

```ts
const cache = new Map();
```

### TSX

#### Don't

```tsx
const _cache = new Map();
```

#### Do

```tsx
const cache = new Map();
```
