---
id: js/no-unreadable-for-of-expression
title: "A for…of loop iterates a name or a simple call, not a nested expression"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the variable that holds the iterable needs a name, which is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-unreadable-for-of-expression]
since: 0.5.0
---

A `for…of` header whose iterable is a call with a call (that has arguments), a `new` expression, a function or an operator inside its arguments — or is itself a conditional, a binary or logical expression, an `await` or a `new` — is reported: name the iterable first. Identifiers, property chains, array literals, simple calls and `Object.keys(x)`-style calls stay. `.map()` and `.filter()` calls are left to `js/no-duplicate-loops`.

**Why:** The loop header is where a reader looks for what is iterated; a nested expression hides it.

```codelaw
utils:
  complex_arg:
    any:
      - kind: call_expression
        has: { field: arguments, has: { nthChild: 1 } }
      - kind: new_expression
      - kind: arrow_function
      - kind: function_expression
      - kind: binary_expression
      - kind: ternary_expression
      - kind: await_expression
      - kind: object
match:
  kind: for_in_statement
  regex: '^for\s*(await\s*)?\(\s*(const|let|var)?\s*[^;]*?\bof\b'
  has:
    field: right
    any:
      - kind: ternary_expression
      - kind: binary_expression
      - kind: await_expression
      - kind: new_expression
      - kind: call_expression
        not: { has: { field: function, kind: member_expression, has: { field: property, regex: '^(map|filter)$' } } }
        has: { field: arguments, has: { matches: complex_arg } }
message: "Name the iterable before the loop"
```

## How to fix

1. Assign the expression to a `const` with a name, and loop over the name.

## Examples

### JavaScript

#### Don't

```js
for (const item of getItems(createArgument(seed))) {
  use(item);
}
```

#### Do

```js
const argument = createArgument(seed);
for (const item of getItems(argument)) {
  use(item);
}
```


### JSX

#### Don't

```jsx
for (const item of getItems(createArgument(seed))) {
  use(item);
}
```

#### Do

```jsx
const argument = createArgument(seed);
for (const item of getItems(argument)) {
  use(item);
}
```


### TypeScript

#### Don't

```ts
for (const item of getItems(createArgument(seed))) {
  use(item);
}
```

#### Do

```ts
const argument = createArgument(seed);
for (const item of getItems(argument)) {
  use(item);
}
```


### TSX

#### Don't

```tsx
for (const item of getItems(createArgument(seed))) {
  use(item);
}
```

#### Do

```tsx
const argument = createArgument(seed);
for (const item of getItems(argument)) {
  use(item);
}
```
