---
id: js/no-async-iterator-callback
title: "No async callback to forEach, filter, some, every or find"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "a loop with await or Promise.all over map changes how the work runs; which one is right is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-async-iterator-callback]
since: 0.5.0
---

`forEach`, `filter`, `some`, `every`, `find` and their kin are not given an `async` callback: they do not wait for its promise, so `forEach` returns before the work ends and `filter` keeps every element (a promise is truthy).

**Why:** The array method treats the returned promise as the answer, which it never is.

```codelaw
match:
  any:
    - pattern: "$A.$M($CB)"
    - pattern: "$A.$M($CB, $$$R)"
constraints:
  M: { regex: '^(forEach|filter|some|every|find|findIndex|findLast|findLastIndex)$' }
  CB:
    any:
      - { kind: arrow_function, regex: '^async\b' }
      - { kind: function_expression, regex: '^async\b' }
message: "`$M` does not wait for an async callback"
```

## How to fix

1. Use a `for … of` loop with `await` inside, or `await Promise.all(items.map(async …))` and work on the results.

## Examples

### JavaScript

#### Don't

```js
items.forEach(async (item) => {
  await save(item);
});
```

#### Do

```js
for (const item of items) {
  await save(item);
}
```


### JSX

#### Don't

```jsx
items.forEach(async (item) => {
  await save(item);
});
```

#### Do

```jsx
for (const item of items) {
  await save(item);
}
```


### TypeScript

#### Don't

```ts
items.forEach(async (item) => {
  await save(item);
});
```

#### Do

```ts
for (const item of items) {
  await save(item);
}
```


### TSX

#### Don't

```tsx
items.forEach(async (item) => {
  await save(item);
});
```

#### Do

```tsx
for (const item of items) {
  await save(item);
}
```
