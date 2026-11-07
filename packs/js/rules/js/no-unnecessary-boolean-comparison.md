---
id: js/no-unnecessary-boolean-comparison
title: "No comparison of a boolean expression with true or false"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-boolean-comparison]
since: 0.5.0
---

An expression that is already a boolean — a comparison, a negation, or a call such as `includes`, `some`, `every`, `has`, `startsWith`, `endsWith`, `Array.isArray` — is not compared with `true` or `false`: `x.includes(y) === true` is `x.includes(y)`, and `=== false` is its negation.

**Why:** The comparison repeats the value it compares.

```codelaw
utils:
  bool-call:
    any:
      - pattern: "$O.$M($$$)"
        has: { field: function, has: { field: property, regex: '^(includes|some|every|has|startsWith|endsWith)$' } }
      - pattern: "Array.isArray($$$)"
      - pattern: "Number.isNaN($$$)"
      - pattern: "Number.isInteger($$$)"
      - pattern: "Number.isFinite($$$)"
  bool-op:
    any:
      - kind: binary_expression
        has: { field: operator, regex: '^(===|!==|==|!=|<|>|<=|>=|instanceof|in)$' }
      - kind: unary_expression
        regex: '^!'
  bool:
    any: [{ matches: bool-call }, { matches: bool-op }]
match:
  any:
    - kind: binary_expression
      all:
        - any: [{ pattern: "$X === true" }, { pattern: "$X == true" }, { pattern: "$X !== false" }, { pattern: "$X != false" }]
        - has: { field: left, matches: bool }
        - pattern: $KEEP
    - kind: binary_expression
      all:
        - any: [{ pattern: "$C === false" }, { pattern: "$C == false" }, { pattern: "$C !== true" }, { pattern: "$C != true" }]
        - has: { field: left, matches: bool-call }
        - pattern: $NEGC
    - kind: binary_expression
      all:
        - any: [{ pattern: "$Y === false" }, { pattern: "$Y == false" }, { pattern: "$Y !== true" }, { pattern: "$Y != true" }]
        - has: { field: left, matches: bool-op }
        - pattern: $NEGB
message: "Drop the comparison with a boolean"
fix:
  - replace: { target: $KEEP, with: "$X" }
  - replace: { target: $NEGC, with: "!$C" }
  - replace: { target: $NEGB, with: "!($Y)" }
```

## Examples

### JavaScript

#### Don't

```js
if (list.includes(item) === true) add(item);
if (Array.isArray(value) === false) wrap(value);
```

#### Do

```js
if (list.includes(item)) add(item);
if (!Array.isArray(value)) wrap(value);
```


### JSX

#### Don't

```jsx
if (list.includes(item) === true) add(item);
if (Array.isArray(value) === false) wrap(value);
```

#### Do

```jsx
if (list.includes(item)) add(item);
if (!Array.isArray(value)) wrap(value);
```


### TypeScript

#### Don't

```ts
if (list.includes(item) === true) add(item);
if (Array.isArray(value) === false) wrap(value);
```

#### Do

```ts
if (list.includes(item)) add(item);
if (!Array.isArray(value)) wrap(value);
```


### TSX

#### Don't

```tsx
if (list.includes(item) === true) add(item);
if (Array.isArray(value) === false) wrap(value);
```

#### Do

```tsx
if (list.includes(item)) add(item);
if (!Array.isArray(value)) wrap(value);
```
