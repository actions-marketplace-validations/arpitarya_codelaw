---
id: js/no-single-promise-in-promise-methods
title: "No Promise.all([single])"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the destructuring around the call changes too"
stability: preview
ported_from: [eslint-plugin-unicorn/no-single-promise-in-promise-methods]
since: 0.5.0
---

`Promise.all`, `Promise.any` and `Promise.race` are not given an array of one element.

**Why:** The wrapper only adds an array around one value.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: ^Promise\.(all|any|race)$
  all:
  - has:
      field: arguments
      has:
        kind: array
        not:
          has:
            nthChild: 2
        has:
          nthChild: 1
          not:
            kind: spread_element
message: One promise needs no `Promise.all`; await it directly
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const [user] = await Promise.all([load()]);
```

#### Do

```js
const user = await load();
```

### JSX

#### Don't

```jsx
const [user] = await Promise.all([load()]);
```

#### Do

```jsx
const user = await load();
```

### TypeScript

#### Don't

```ts
const [user] = await Promise.all([load()]);
```

#### Do

```ts
const user = await load();
```

### TSX

#### Don't

```tsx
const [user] = await Promise.all([load()]);
```

#### Do

```tsx
const user = await load();
```
