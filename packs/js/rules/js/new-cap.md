---
id: js/new-cap
title: "Constructors are capitalised"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "renaming the constructor changes its declaration and every use"
stability: preview
ported_from: [eslint/new-cap]
since: 0.5.0
---

A function called with `new` has a name starting with a capital letter. Upstream also reports capitalised functions called without `new`; this port checks `new`.

**Why:** The capital is the only sign at the call site that `new` is required.

```codelaw

match:
  kind: new_expression
  has: { field: constructor, any: [{kind: identifier, regex: '^[a-z_$]'}, {kind: member_expression, has: { field: property, regex: '^[a-z_$]' }}], pattern: $C }
message: "`new $C`: a constructor's name starts with a capital letter"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const u = new user();
```

#### Do

```js
const u = new User();
```

### JSX

#### Don't

```jsx
const u = new user();
```

#### Do

```jsx
const u = new User();
```

### TypeScript

#### Don't

```ts
const u = new user();
```

#### Do

```ts
const u = new User();
```

### TSX

#### Don't

```tsx
const u = new user();
```

#### Do

```tsx
const u = new User();
```
