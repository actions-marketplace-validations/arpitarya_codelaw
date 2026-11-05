---
id: js/no-extra-bind
title: "No bind() on a function that does not use this"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/no-extra-bind]
since: 0.5.0
---

`.bind(thisArg)` is not called on an arrow function, or on a function expression that does not read `this`.

**Why:** The bound `this` is never read, so the call only allocates another function.

```codelaw

match:
  kind: call_expression
  pattern: $F.bind($T)
  has:
    field: function
    has:
      field: object
      kind: parenthesized_expression
      has:
        any:
        - kind: arrow_function
        - kind: function_expression
          not: { has: { stopBy: { any: [{kind: function_expression}, {kind: function_declaration}, {kind: class}] }, any: [{kind: this}, {kind: identifier, regex: '^arguments$'}] } }
        pattern: $FN
  not: { has: { field: arguments, has: { any: [{kind: call_expression}, {kind: new_expression}, {kind: assignment_expression}, {kind: update_expression}] } } }
message: "This function does not use `this`; `.bind()` is not needed"
fix: "$FN"
```

## Examples

### JavaScript

#### Don't

```js
const run = (() => go()).bind(this);
```

#### Do

```js
const run = () => go();
```

### JSX

#### Don't

```jsx
const run = (() => go()).bind(this);
```

#### Do

```jsx
const run = () => go();
```

### TypeScript

#### Don't

```ts
const run = (() => go()).bind(this);
```

#### Do

```ts
const run = () => go();
```

### TSX

#### Don't

```tsx
const run = (() => go()).bind(this);
```

#### Do

```tsx
const run = () => go();
```
