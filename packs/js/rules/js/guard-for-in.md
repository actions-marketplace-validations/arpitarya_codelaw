---
id: js/guard-for-in
title: "for-in is filtered"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the filter (own keys, or a loop over `Object.keys`) is the author's choice"
stability: preview
ported_from: [eslint/guard-for-in]
since: 0.5.0
---

The body of a `for…in` loop starts with an `if` that filters the keys.

**Why:** `for…in` also visits inherited enumerable properties.

```codelaw
match:
  kind: for_in_statement
  all:
  - has: {field: operator, regex: ^in$}
  - has:
      field: body
      not:
        any:
        - {kind: if_statement}
        - kind: statement_block
          has: {kind: if_statement, nthChild: 1}
        - {kind: statement_block, regex: '^\{\s*\}$'}
message: Filter a `for…in` loop with an `if`, or loop over `Object.keys`
```

## How to fix

1. Loop over `Object.keys(obj)` instead, or start the body with `if (!Object.hasOwn(obj, key)) continue;`.

## Examples

### JavaScript

#### Don't

```js
for (const key in obj) {
  use(obj[key]);
}
```

#### Do

```js
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

### JSX

#### Don't

```jsx
for (const key in obj) {
  use(obj[key]);
}
```

#### Do

```jsx
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

### TypeScript

#### Don't

```ts
for (const key in obj) {
  use(obj[key]);
}
```

#### Do

```ts
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```

### TSX

#### Don't

```tsx
for (const key in obj) {
  use(obj[key]);
}
```

#### Do

```tsx
for (const key of Object.keys(obj)) {
  use(obj[key]);
}
```
