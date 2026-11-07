---
id: js/func-name-matching
title: "A named function expression has the name it is assigned to"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two names is right is the author's"
fix_exception: value
stability: preview
ported_from: [eslint/func-name-matching]
since: 0.5.0
---

A function expression with a name, assigned to a variable, a property or an object key, has that same name: `const save = function save() {}`, not `function store() {}`.

**Why:** Stack traces show the function's own name; when it differs from the binding, the trace points nowhere a reader can search.

```codelaw
match:
  any:
    - kind: variable_declarator
      has: { field: value, kind: function_expression, has: { field: name } }
      not: { pattern: { context: "var $X = function $X($$$P) { $$$S }", selector: variable_declarator } }
    - pattern: "$O.$A = function $B($$$P) { $$$S }"
      not: { pattern: "$Q.$X = function $X($$$R) { $$$T }" }
    - kind: pair
      has: { field: value, kind: function_expression, has: { field: name } }
      not: { pattern: { context: "({ $X: function $X($$$P) { $$$S } })", selector: pair } }
message: "The function's name differs from the name it is assigned to"
```

## How to fix

1. Give the function expression the name it is assigned to, or drop its own name so it takes that one.

## Examples

### JavaScript

#### Don't

```js
const save = function store(record) {
  db.put(record);
};
```

#### Do

```js
const save = function save(record) {
  db.put(record);
};
```


### JSX

#### Don't

```jsx
const save = function store(record) {
  db.put(record);
};
```

#### Do

```jsx
const save = function save(record) {
  db.put(record);
};
```


### TypeScript

#### Don't

```ts
const save = function store(record) {
  db.put(record);
};
```

#### Do

```ts
const save = function save(record) {
  db.put(record);
};
```


### TSX

#### Don't

```tsx
const save = function store(record) {
  db.put(record);
};
```

#### Do

```tsx
const save = function save(record) {
  db.put(record);
};
```
