---
id: js/no-unreadable-iife
title: "No arrow IIFE with a parenthesized body"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "naming the function or the value is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-unreadable-iife]
since: 0.5.0
---

An arrow function that is called at once does not return a parenthesized expression: `(x => (x ? a : b))(value)` hides its argument behind two layers of parentheses.

**Why:** Reading it means pairing the parentheses before knowing what is called with what.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: parenthesized_expression
    has: { kind: arrow_function, has: { field: body, kind: parenthesized_expression } }
message: "Name this function or compute the value in a statement"
```

## How to fix

1. Assign the argument to a variable and write the expression on it, or give the function a name and call it.

## Examples

### JavaScript

#### Don't

```js
const name = ((user) => (user ? user.name : "guest"))(getUser());
```

#### Do

```js
const user = getUser();
const name = user ? user.name : "guest";
```


### JSX

#### Don't

```jsx
const name = ((user) => (user ? user.name : "guest"))(getUser());
```

#### Do

```jsx
const user = getUser();
const name = user ? user.name : "guest";
```


### TypeScript

#### Don't

```ts
const name = ((user) => (user ? user.name : "guest"))(getUser());
```

#### Do

```ts
const user = getUser();
const name = user ? user.name : "guest";
```


### TSX

#### Don't

```tsx
const name = ((user) => (user ? user.name : "guest"))(getUser());
```

#### Do

```tsx
const user = getUser();
const name = user ? user.name : "guest";
```
