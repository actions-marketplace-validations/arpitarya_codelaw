---
id: js/no-identical-conditions
title: "No repeated condition in an if chain"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which condition was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-identical-conditions]
since: 0.5.0
---

An `if … else if` chain does not test the same condition twice (checked for the next two links).

**Why:** The second branch is dead code: the first one always takes its cases.

```codelaw
match:
  kind: if_statement
  has:
    field: condition
    pattern: $C
  all:
  - has:
      field: alternative
      has:
        kind: if_statement
        any:
        - has:
            field: condition
            pattern: $C
        - has:
            field: alternative
            has:
              kind: if_statement
              has:
                field: condition
                pattern: $C
message: This condition is tested again further down the chain; the later branch can never run
```

## How to fix

1. Correct the repeated condition, or remove the dead branch.

## Examples

### JavaScript

#### Don't

```js
if (x === 1) {
  a();
} else if (x === 1) {
  b();
}
```

#### Do

```js
if (x === 1) {
  a();
} else if (x === 2) {
  b();
}
```

### JSX

#### Don't

```jsx
if (x === 1) {
  a();
} else if (x === 1) {
  b();
}
```

#### Do

```jsx
if (x === 1) {
  a();
} else if (x === 2) {
  b();
}
```

### TypeScript

#### Don't

```ts
if (x === 1) {
  a();
} else if (x === 1) {
  b();
}
```

#### Do

```ts
if (x === 1) {
  a();
} else if (x === 2) {
  b();
}
```

### TSX

#### Don't

```tsx
if (x === 1) {
  a();
} else if (x === 1) {
  b();
}
```

#### Do

```tsx
if (x === 1) {
  a();
} else if (x === 2) {
  b();
}
```
