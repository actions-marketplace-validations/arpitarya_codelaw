---
id: js/no-keyword-prefix
title: "No new or class prefix in names"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-keyword-prefix]
since: 0.5.0
---

A declared name does not begin with the keyword `new` or `class` followed by a capital (the upstream default list).

**Why:** `newUser` reads like `new User` at a glance.

```codelaw

match:
  kind: identifier
  regex: '^(new|class)[A-Z_]'
  any:
    - inside: { kind: variable_declarator, field: name }
    - inside: { kind: function_declaration, field: name }
message: "A name starting with `new` or `class` reads like the keyword; rename it"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const newUser = create();
```

#### Do

```js
const createdUser = create();
```

### JSX

#### Don't

```jsx
const newUser = create();
```

#### Do

```jsx
const createdUser = create();
```

### TypeScript

#### Don't

```ts
const newUser = create();
```

#### Do

```ts
const createdUser = create();
```

### TSX

#### Don't

```tsx
const newUser = create();
```

#### Do

```tsx
const createdUser = create();
```
