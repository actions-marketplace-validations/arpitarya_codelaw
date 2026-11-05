---
id: js/prefer-then-catch
title: ".catch() rather than a second then() argument"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "`.catch()` also catches errors from `show`, which changes behaviour"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-then-catch]
since: 0.5.0
---

`.then()` is not given a rejection handler as its second argument.

**Why:** The second argument does not catch an error thrown by the first callback, which `.catch()` would.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^then$
  all:
  - has:
      field: arguments
      has:
        nthChild: 2
message: Handle rejections with `.catch()`; a second `then` argument misses errors thrown by the first
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
load().then(show, report);
```

#### Do

```js
load().then(show).catch(report);
```

### JSX

#### Don't

```jsx
load().then(show, report);
```

#### Do

```jsx
load().then(show).catch(report);
```

### TypeScript

#### Don't

```ts
load().then(show, report);
```

#### Do

```ts
load().then(show).catch(report);
```

### TSX

#### Don't

```tsx
load().then(show, report);
```

#### Do

```tsx
load().then(show).catch(report);
```
