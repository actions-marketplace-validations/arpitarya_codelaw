---
id: js/prefer-object-destructuring-defaults
title: "Destructuring defaults rather than a defaults object spread under the options"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a default in the pattern applies only to `undefined`, while the spread form keeps an explicit `undefined` from the options; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-object-destructuring-defaults]
since: 0.5.0
---

`const { a } = { a: 1, ...options }` builds a temporary object to supply a default; `const { a = 1 } = { ...options }` puts the default next to the binding. This rule handles a pattern of one or two shorthand properties whose defaults are listed, in the same order, before a final spread.

**Why:** Each default sits beside the name it is for.

```codelaw
match:
  kind: lexical_declaration
  any:
    - all:
        - any:
            - pattern: "const { $A } = { $A: $X, ...$O };"
            - pattern: "let { $A } = { $A: $X, ...$O };"
        - has: { kind: variable_declarator, pattern: $W1 }
    - all:
        - any:
            - pattern: "const { $A, $B } = { $A: $X, $B: $Y, ...$O };"
            - pattern: "let { $A, $B } = { $A: $X, $B: $Y, ...$O };"
        - has: { kind: variable_declarator, pattern: $W2 }
message: "Put the defaults in the destructuring pattern"
fix:
  - replace: { target: $W1, with: "{ $A = $X } = { ...$O }" }
  - replace: { target: $W2, with: "{ $A = $X, $B = $Y } = { ...$O }" }
```

## Examples

### JavaScript

#### Don't

```js
const { retries, delay } = { retries: 3, delay: 100, ...options };
```

#### Do

```js
const { retries = 3, delay = 100 } = { ...options };
```


### JSX

#### Don't

```jsx
const { retries, delay } = { retries: 3, delay: 100, ...options };
```

#### Do

```jsx
const { retries = 3, delay = 100 } = { ...options };
```


### TypeScript

#### Don't

```ts
const { retries, delay } = { retries: 3, delay: 100, ...options };
```

#### Do

```ts
const { retries = 3, delay = 100 } = { ...options };
```


### TSX

#### Don't

```tsx
const { retries, delay } = { retries: 3, delay: 100, ...options };
```

#### Do

```tsx
const { retries = 3, delay = 100 } = { ...options };
```
