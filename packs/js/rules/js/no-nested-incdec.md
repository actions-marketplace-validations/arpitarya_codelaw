---
id: js/no-nested-incdec
title: "No ++ or -- inside another expression"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the statement depends on prefix or postfix"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-nested-incdec]
since: 0.5.0
---

An increment or decrement is a statement on its own (or a `for` update), not part of a larger expression.

**Why:** Whether the old or the new value is used depends on `i++` versus `++i`, which is easy to misread.

```codelaw
match:
  kind: update_expression
  not:
    inside:
      any:
      - kind: expression_statement
      - kind: sequence_expression
  all:
  - not:
      inside:
        kind: for_statement
        field: increment
message: Put the increment on its own line
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const next = items[i++];
```

#### Do

```js
const next = items[i];
i++;
```

### JSX

#### Don't

```jsx
const next = items[i++];
```

#### Do

```jsx
const next = items[i];
i++;
```

### TypeScript

#### Don't

```ts
const next = items[i++];
```

#### Do

```ts
const next = items[i];
i++;
```

### TSX

#### Don't

```tsx
const next = items[i++];
```

#### Do

```tsx
const next = items[i];
i++;
```
