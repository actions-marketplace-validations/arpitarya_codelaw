---
id: js/class-name
title: "Class names in PascalCase"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's choice and changes every use"
stability: preview
ported_from: [eslint-plugin-sonarjs/class-name]
since: 0.5.0
---

A class name matches `^[A-Z][a-zA-Z0-9]*$` (the upstream default).

**Why:** One naming style tells classes from functions and values at a glance.

```codelaw

match:
  any: [{kind: class_declaration}, {kind: class}]
  has: { field: name, not: { regex: '^[A-Z][a-zA-Z0-9]*$' }, pattern: $N }
message: "Name the class in PascalCase"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
class user_record {}
```

#### Do

```js
class UserRecord {}
```

### JSX

#### Don't

```jsx
class user_record {}
```

#### Do

```jsx
class UserRecord {}
```

### TypeScript

#### Don't

```ts
class user_record {}
```

#### Do

```ts
class UserRecord {}
```

### TSX

#### Don't

```tsx
class user_record {}
```

#### Do

```tsx
class UserRecord {}
```
