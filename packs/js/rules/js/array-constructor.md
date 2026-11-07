---
id: js/array-constructor
title: "An array literal rather than the Array constructor"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-sonarjs/array-constructor, eslint/no-array-constructor]
since: 0.5.0
---

An array of given elements is written `[a, b]`, not `new Array(a, b)` or `Array(a, b)`, and an empty one `[]`. `Array(n)` with one argument, which makes an array of that length, is left alone.

**Why:** The constructor means two different things depending on how many arguments it gets; the literal means one.

```codelaw
match:
  any:
    - pattern: "new $C()"
      all: [{ pattern: $EMPTY }]
    - pattern: "$C()"
      all: [{ pattern: $EMPTY }]
    - pattern: "new $C($A, $$$R)"
      not: { pattern: "new $C($Z)" }
      all: [{ pattern: $MANY }]
    - pattern: "$C($A, $$$R)"
      not: { pattern: "$C($Z)" }
      all: [{ pattern: $MANY }]
constraints:
  C: { kind: identifier, regex: '^Array$' }
where:
  $C: builtin
message: "Use an array literal"
fix:
  - replace: { target: $EMPTY, with: "[]" }
  - replace: { target: $MANY, with: "[$A, $$$R]" }
```

## Examples

### JavaScript

#### Don't

```js
const pair = new Array(left, right);
```

#### Do

```js
const pair = [left, right];
```


### JSX

#### Don't

```jsx
const pair = new Array(left, right);
```

#### Do

```jsx
const pair = [left, right];
```


### TypeScript

#### Don't

```ts
const pair = new Array(left, right);
```

#### Do

```ts
const pair = [left, right];
```


### TSX

#### Don't

```tsx
const pair = new Array(left, right);
```

#### Do

```tsx
const pair = [left, right];
```
