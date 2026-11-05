---
id: js/no-array-method-this-argument
title: "No thisArg to array callbacks"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a regular function callback that reads `this` would lose it"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-method-this-argument]
since: 0.5.0
---

The array methods that take a callback are not given their second, `thisArg`, argument.

**Why:** An arrow function ignores `thisArg`, so it is dead; for a regular function, a closure or `bind` is clearer.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^(every|filter|find|findLast|findIndex|findLastIndex|flatMap|forEach|map|some)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        any:
        - kind: arrow_function
        - kind: identifier
        - kind: member_expression
      not:
        has:
          nthChild: 3
      all:
      - has:
          nthChild: 2
          pattern: $T
message: Don't pass a `this` argument to an array callback; bind it or use an arrow function
fix:
- remove: $T
```

## Examples

### JavaScript

#### Don't

```js
items.map((x) => x * 2, context);
```

#### Do

```js
items.map((x) => x * 2);
```

### JSX

#### Don't

```jsx
items.map((x) => x * 2, context);
```

#### Do

```jsx
items.map((x) => x * 2);
```

### TypeScript

#### Don't

```ts
items.map((x) => x * 2, context);
```

#### Do

```ts
items.map((x) => x * 2);
```

### TSX

#### Don't

```tsx
items.map((x) => x * 2, context);
```

#### Do

```tsx
items.map((x) => x * 2);
```
