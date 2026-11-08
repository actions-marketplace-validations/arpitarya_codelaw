---
id: js/prefer-short-arrow-method
title: "An arrow property rather than an object method that only returns"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-short-arrow-method]
since: 0.5.0
---

An object-literal method whose body is a single `return` of an expression is written as an arrow-function property: `{ getUrl() { return url; } }` is `{ getUrl: () => url }`. Methods that use `this`, `arguments`, `super` or `new.target`, generators, getters, setters and class methods are left: an arrow would change them.

**Why:** The arrow says the method is a plain function of its arguments, in less text.

```codelaw
match:
  kind: method_definition
  inside: { kind: object }
  has: { field: body, kind: statement_block, has: { nthChild: { position: 1, ofRule: { not: { kind: comment } } }, kind: return_statement, has: { pattern: $E } } }
  not:
    any:
      - has: { field: body, has: { nthChild: 2 } }
      - has: { field: body, has: { kind: comment } }
      - regex: '^(get|set|static)\s'
      - regex: '^\*'
      - regex: '^async\s*\*'
      - has: { stopBy: end, any: [{ kind: this }, { kind: super }, { kind: meta_property }, { kind: identifier, regex: '^arguments$' }] }
  any:
    - all:
        - regex: '^async\s'
        - has: { field: name, pattern: $N1 }
        - has: { field: parameters, pattern: $P1 }
        - pattern: $W1
    - all:
        - not: { regex: '^async\s' }
        - has: { field: name, pattern: $N2 }
        - has: { field: parameters, pattern: $P2 }
        - pattern: $W2
message: "Write this method as an arrow property"
fix:
  - replace: { target: $W1, with: "$N1: async $P1 => $E|arrow-body" }
  - replace: { target: $W2, with: "$N2: $P2 => $E|arrow-body" }
```

## Examples

### JavaScript

#### Don't

```js
const api = {
  add(a, b) {
    return a + b;
  },
};
```

#### Do

```js
const api = {
  add: (a, b) => a + b,
};
```


### JSX

#### Don't

```jsx
const api = {
  add(a, b) {
    return a + b;
  },
};
```

#### Do

```jsx
const api = {
  add: (a, b) => a + b,
};
```


### TypeScript

#### Don't

```ts
const api = {
  add(a, b) {
    return a + b;
  },
};
```

#### Do

```ts
const api = {
  add: (a, b) => a + b,
};
```


### TSX

#### Don't

```tsx
const api = {
  add(a, b) {
    return a + b;
  },
};
```

#### Do

```tsx
const api = {
  add: (a, b) => a + b,
};
```
