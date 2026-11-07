---
id: js/no-for-each
title: "for…of rather than forEach"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`for … of` iterates what the value's iterator gives, where `forEach` visits an array's indices: a sparse array, or a `Map` (whose `forEach` passes the value and `for … of` the entry), changes; a callback with a `return`, an index parameter, `async`, or a block-less body is rewritten by hand"
stability: preview
ported_from: [eslint-plugin-unicorn/no-for-each]
since: 0.5.0
---

A statement calling `.forEach` with an arrow function is written as a `for … of` loop.

**Why:** A loop can `break`, `continue`, `return` and `await`; a `forEach` callback can do none of them.

```codelaw
match:
  kind: expression_statement
  has:
    kind: call_expression
    has:
      field: function
      kind: member_expression
      has:
        field: property
        regex: ^forEach$
    all:
    - has:
        field: arguments
        has:
          nthChild: 1
          kind: arrow_function
  any:
    - all:
        - pattern: $STMT
        - has:
            kind: call_expression
            any: [{ pattern: "$A.forEach(($X) => $B)" }, { pattern: "$A.forEach($X => $B)" }]
            has:
              field: arguments
              has:
                kind: arrow_function
                not: { regex: ^async }
                has:
                  field: body
                  kind: statement_block
                  not: { has: { stopBy: { any: [{ kind: function_expression }, { kind: function_declaration }, { kind: arrow_function }, { kind: method_definition }, { kind: class }] }, kind: return_statement } }
    - regex: ''
message: Use a `for … of` loop instead of `.forEach`
fix:
  - replace: { target: $STMT, with: "for (const $X of $A) $B" }
```

## How to fix

1. Rewrite as `for (const item of items) { … }`; turn `return` into `continue`; for an index parameter use `for (const [i, item] of items.entries())`.

## Examples

### JavaScript

#### Don't

```js
items.forEach((item) => {
  send(item);
});
```

#### Do

```js
for (const item of items) {
  send(item);
}
```

### JSX

#### Don't

```jsx
items.forEach((item) => {
  send(item);
});
```

#### Do

```jsx
for (const item of items) {
  send(item);
}
```

### TypeScript

#### Don't

```ts
items.forEach((item) => {
  send(item);
});
```

#### Do

```ts
for (const item of items) {
  send(item);
}
```

### TSX

#### Don't

```tsx
items.forEach((item) => {
  send(item);
});
```

#### Do

```tsx
for (const item of items) {
  send(item);
}
```
