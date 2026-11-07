---
id: js/require-proxy-trap-boolean-return
title: "A Proxy set or deleteProperty trap returns a boolean"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`return true` is offered; a trap that forwards may mean to return `Reflect.set`'s result"
stability: preview
ported_from: [eslint-plugin-unicorn/require-proxy-trap-boolean-return]
since: 0.5.0
---

A `set`, `deleteProperty` or `defineProperty` trap of a `new Proxy` handler returns whether it succeeded.

**Why:** A trap that returns nothing returns `undefined`, which strict-mode code takes as failure and throws a `TypeError`.

```codelaw
utils:
  boundary:
    any:
      - kind: function_expression
      - kind: arrow_function
      - kind: function_declaration
      - kind: method_definition
match:
  kind: method_definition
  has: { field: name, regex: '^(set|deleteProperty|defineProperty)$' }
  inside:
    kind: object
    inside: { kind: arguments, inside: { kind: new_expression, has: { field: constructor, regex: '^Proxy$' } } }
  not:
    has:
      field: body
      has: { stopBy: { matches: boundary }, kind: return_statement }
message: "This trap returns nothing; return `true` when it succeeds"
fix:
  - edit-text: { pattern: '(\n([ \t]*))\}$', replace: '${1}  return true;${1}}' }
```

## Examples

### JavaScript

#### Don't

```js
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
  },
});
```

#### Do

```js
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
    return true;
  },
});
```


### JSX

#### Don't

```jsx
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
  },
});
```

#### Do

```jsx
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
    return true;
  },
});
```


### TypeScript

#### Don't

```ts
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
  },
});
```

#### Do

```ts
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
    return true;
  },
});
```


### TSX

#### Don't

```tsx
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
  },
});
```

#### Do

```tsx
const p = new Proxy(target, {
  set(obj, key, value) {
    obj[key] = value;
    return true;
  },
});
```
