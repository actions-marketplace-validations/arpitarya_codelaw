---
id: js/prefer-type-error
title: "TypeError for a failed type check"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-type-error]
since: 0.5.0
---

An `if` whose whole condition is a type check (`typeof`, `instanceof`, `Array.isArray`) and whose branch throws, throws a `TypeError`, not a plain `Error`.

**Why:** A `TypeError` says what went wrong — a value of the wrong type — and callers can tell it apart from other failures.

```codelaw
utils:
  type-check:
    any:
      - kind: binary_expression
        has: { kind: unary_expression, regex: '^typeof\b' }
      - pattern: "$V instanceof $C"
      - pattern: "!($V instanceof $C)"
      - pattern: "Array.isArray($V)"
      - pattern: "!Array.isArray($V)"
  guarded:
    kind: if_statement
    has:
      field: condition
      has: { matches: type-check }
match:
  pattern: "new Error($$$A)"
  inside:
    kind: throw_statement
    any:
      - inside: { matches: guarded, field: consequence }
      - inside:
          kind: statement_block
          inside: { matches: guarded, field: consequence }
message: "Throw a `TypeError` when a type check fails"
fix: "new TypeError($$$A)"
```

## Examples

### JavaScript

#### Don't

```js
if (typeof name !== "string") {
  throw new Error("name must be a string");
}
```

#### Do

```js
if (typeof name !== "string") {
  throw new TypeError("name must be a string");
}
```


### JSX

#### Don't

```jsx
if (typeof name !== "string") {
  throw new Error("name must be a string");
}
```

#### Do

```jsx
if (typeof name !== "string") {
  throw new TypeError("name must be a string");
}
```


### TypeScript

#### Don't

```ts
if (typeof name !== "string") {
  throw new Error("name must be a string");
}
```

#### Do

```ts
if (typeof name !== "string") {
  throw new TypeError("name must be a string");
}
```


### TSX

#### Don't

```tsx
if (typeof name !== "string") {
  throw new Error("name must be a string");
}
```

#### Do

```tsx
if (typeof name !== "string") {
  throw new TypeError("name must be a string");
}
```
