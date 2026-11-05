---
id: js/promise-no-promise-in-callback
title: "No promise chain inside a node-style callback"
languages: [javascript, jsx, typescript, tsx]
severity: warn
fix: manual
fix_reason: "converting the callback API to promises (for example with `util.promisify`) is a refactor of the caller"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-promise/no-promise-in-callback]
since: 0.5.0
---

A callback whose first parameter is `err` or `error` does not start a `.then()` or `.catch()` chain.

**Why:** Mixing the two styles splits error handling in two places, and a rejection inside the callback goes nowhere.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: member_expression, has: { field: property, regex: '^(then|catch)$' } }
  inside:
    stopBy: end
    any: [{kind: arrow_function}, {kind: function_expression}]
    has:
      kind: formal_parameters
      has:
        any: [{kind: identifier}, {kind: required_parameter}]
        nthChild: 1
        regex: '^err(or)?\b'
message: "Avoid promises inside a node-style callback; promisify the callback API instead"
```

## How to fix

1. Wrap the callback API with `util.promisify` (or write a promise version) and chain or `await` it.

## Examples

### JavaScript

#### Don't

```js
fs.readFile(path, (err, text) => {
  parse(text).then(show);
});
```

#### Do

```js
const text = await readFile(path);
show(await parse(text));
```

### JSX

#### Don't

```jsx
fs.readFile(path, (err, text) => {
  parse(text).then(show);
});
```

#### Do

```jsx
const text = await readFile(path);
show(await parse(text));
```

### TypeScript

#### Don't

```ts
fs.readFile(path, (err, text) => {
  parse(text).then(show);
});
```

#### Do

```ts
const text = await readFile(path);
show(await parse(text));
```

### TSX

#### Don't

```tsx
fs.readFile(path, (err, text) => {
  parse(text).then(show);
});
```

#### Do

```tsx
const text = await readFile(path);
show(await parse(text));
```
