---
id: js/array-callback-return
title: "A callback to map, filter, reduce and the like returns a value"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the callback should return is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/array-callback-return]
since: 0.5.0
---

A block-bodied callback passed to an array method that uses its return value — `map`, `filter`, `find`, `findIndex`, `findLast`, `findLastIndex`, `every`, `some`, `reduce`, `reduceRight`, `flatMap`, `sort`, `toSorted`, `Array.from`'s mapper — returns a value. ESLint follows every code path; this port reports a callback that has no `return` with a value at all (one that only throws is left alone).

**Why:** Without a `return` the callback gives `undefined`, so `map` fills the array with it and `filter` keeps nothing.

```codelaw
utils:
  boundary:
    any:
      - kind: function_expression
      - kind: arrow_function
      - kind: function_declaration
      - kind: generator_function
      - kind: generator_function_declaration
      - kind: method_definition
      - kind: class_body
  no-value-return:
    any:
      - kind: arrow_function
      - kind: function_expression
    has:
      field: body
      kind: statement_block
      not:
        has:
          stopBy: { matches: boundary }
          any:
            - { kind: return_statement, regex: '^return\s*[^;\s]' }
            - kind: throw_statement
match:
  any:
    - pattern: "$A.$M($CB)"
    - pattern: "$A.$M($CB, $$$R)"
    - pattern: "Array.from($X, $CB)"
constraints:
  M: { regex: '^(map|filter|find|findIndex|findLast|findLastIndex|every|some|reduce|reduceRight|flatMap|sort|toSorted)$' }
  CB: { matches: no-value-return }
message: "This callback returns nothing; `$M` uses what it returns"
```

## How to fix

1. Return the value the method expects from every branch of the callback, or use `forEach` or a loop if no value is wanted.

## Examples

### JavaScript

#### Don't

```js
const names = users.map((user) => {
  user.name.trim();
});
```

#### Do

```js
const names = users.map((user) => {
  return user.name.trim();
});
```


### JSX

#### Don't

```jsx
const names = users.map((user) => {
  user.name.trim();
});
```

#### Do

```jsx
const names = users.map((user) => {
  return user.name.trim();
});
```


### TypeScript

#### Don't

```ts
const names = users.map((user) => {
  user.name.trim();
});
```

#### Do

```ts
const names = users.map((user) => {
  return user.name.trim();
});
```


### TSX

#### Don't

```tsx
const names = users.map((user) => {
  user.name.trim();
});
```

#### Do

```tsx
const names = users.map((user) => {
  return user.name.trim();
});
```
