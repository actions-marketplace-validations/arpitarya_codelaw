---
id: js/import-no-anonymous-default-export
title: "Default exports have a name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the name to give the value is the author's choice"
stability: preview
ported_from: [eslint-plugin-import/no-anonymous-default-export]
since: 0.5.0
---

`export default` exports a named function or class, or a name declared earlier; not an arrow function, an anonymous function or class, an object, an array, a literal or a `new` expression. A call is allowed, as upstream allows it by default.

**Why:** An anonymous default export has no name to search for, auto-import guesses one, and stack traces and dev tools show it as `default` or `anonymous`.

```codelaw
match:
  kind: export_statement
  has:
    field: value
    any:
      - kind: arrow_function
      - kind: function_expression
        not: { has: { field: name, regex: "." } }
      - kind: class
        not: { has: { field: name, regex: "." } }
      - kind: object
      - kind: array
      - kind: new_expression
      - kind: string
      - kind: number
      - kind: regex
      - kind: "true"
      - kind: "false"
      - kind: "null"
message: "This default export has no name; declare it under a name and export that"
```

## How to fix

1. Give the value a name that says what it is: `const routes = [...]` or `function createStore() {}`.
2. Export that name: `export default routes;`, or write `export default function createStore() {}`.

## Examples

### JavaScript

#### Don't

```js
export default () => {
  return 1;
};
```

#### Do

```js
export default function one() {
  return 1;
}
```

### JSX

#### Don't

```jsx
export default () => {
  return 1;
};
```

#### Do

```jsx
export default function one() {
  return 1;
}
```

### TypeScript

#### Don't

```ts
export default () => {
  return 1;
};
```

#### Do

```ts
export default function one() {
  return 1;
}
```

### TSX

#### Don't

```tsx
export default () => {
  return 1;
};
```

#### Do

```tsx
export default function one() {
  return 1;
}
```
