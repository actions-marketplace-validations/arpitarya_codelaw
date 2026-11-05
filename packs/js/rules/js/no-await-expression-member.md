---
id: js/no-await-expression-member
title: "No member access on an await expression"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the destructuring depends on how the value is used next"
stability: preview
ported_from: [eslint-plugin-unicorn/no-await-expression-member]
since: 0.5.0
---

A property is not read off a parenthesized `await` (`(await x).y`).

**Why:** The parentheses are easy to misplace and hard to read; a destructuring or a variable says the same plainly.

```codelaw

match:
  kind: member_expression
  has: { field: object, kind: parenthesized_expression, has: { kind: await_expression } }
message: "Assign the awaited value to a variable, then read the property"
```

## How to fix

1. Destructure the awaited value (`const { name } = await getUser()`) or assign it to a variable first.

## Examples

### JavaScript

#### Don't

```js
const name = (await getUser()).name;
```

#### Do

```js
const { name } = await getUser();
```

### JSX

#### Don't

```jsx
const name = (await getUser()).name;
```

#### Do

```jsx
const { name } = await getUser();
```

### TypeScript

#### Don't

```ts
const name = (await getUser()).name;
```

#### Do

```ts
const { name } = await getUser();
```

### TSX

#### Don't

```tsx
const name = (await getUser()).name;
```

#### Do

```tsx
const { name } = await getUser();
```
