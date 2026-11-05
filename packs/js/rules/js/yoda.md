---
id: js/yoda
title: "Variable first in a comparison"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "swapping the operands changes their evaluation order, which matters if either has a side effect"
stability: preview
ported_from: [eslint/yoda]
since: 0.5.0
---

An equality comparison does not put the literal on the left (`"red" === color`). Relational comparisons (`<`, `>`) are left alone in this port.

**Why:** Reading "if red is the color" is backwards; the variable comes first in speech and in code.

```codelaw
match:
  kind: binary_expression
  has:
    field: operator
    regex: ^(===|!==|==|!=)$
  all:
  - has:
      field: left
      any:
      - kind: string
      - kind: number
      - kind: 'null'
      - kind: 'true'
      - kind: 'false'
      - kind: template_string
        not:
          has:
            kind: template_substitution
      pattern: $L
  - has:
      field: right
      not:
        any:
        - kind: string
        - kind: number
        - kind: 'null'
        - kind: 'true'
        - kind: 'false'
        - kind: template_string
      pattern: $R
message: 'Put the value being tested first: `$R` before the literal'
fix:
- replace:
    target: $L
    with: $R
- replace:
    target: $R
    with: $L
```

## Examples

### JavaScript

#### Don't

```js
if ("admin" === role) {
  grant();
}
```

#### Do

```js
if (role === "admin") {
  grant();
}
```

### JSX

#### Don't

```jsx
if ("admin" === role) {
  grant();
}
```

#### Do

```jsx
if (role === "admin") {
  grant();
}
```

### TypeScript

#### Don't

```ts
if ("admin" === role) {
  grant();
}
```

#### Do

```ts
if (role === "admin") {
  grant();
}
```

### TSX

#### Don't

```tsx
if ("admin" === role) {
  grant();
}
```

#### Do

```tsx
if (role === "admin") {
  grant();
}
```
