---
id: js/no-useless-logical-operand
title: "No boolean literal that decides nothing in && or ||"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-logical-operand]
since: 0.5.0
---

A leading `true &&` or `false ||` leaves the other operand as the value, so it is dropped; a leading `false &&` or `true ||` is the whole value, so the expression is that literal. In a condition, a trailing `&& true` or `|| false` is dropped too, since only truthiness is read there.

**Why:** The literal makes a reader check whether it does anything; it doesn't.

```codelaw
utils:
  cond:
    inside:
      kind: parenthesized_expression
      inside: { any: [{ kind: if_statement }, { kind: while_statement }, { kind: do_statement }], field: condition }
match:
  kind: binary_expression
  not: { inside: { kind: binary_expression } }
  any:
    - all:
        - any:
            - pattern: "true && $X"
            - pattern: "false || $X"
        - pattern: $W1
    - all:
        - any:
            - pattern: "false && $Y"
            - pattern: "true || $Y"
        - has: { field: left, pattern: $L }
        - pattern: $W2
    - all:
        - any:
            - pattern: "$Z && true"
            - pattern: "$Z || false"
        - matches: cond
        - pattern: $W3
message: "This boolean literal changes nothing"
fix:
  - replace: { target: $W1, with: "$X" }
  - replace: { target: $W2, with: "$L" }
  - replace: { target: $W3, with: "$Z" }
```

## Examples

### JavaScript

#### Don't

```js
const value = true && input;
```

#### Do

```js
const value = input;
```


### JSX

#### Don't

```jsx
const value = true && input;
```

#### Do

```jsx
const value = input;
```


### TypeScript

#### Don't

```ts
const value = true && input;
```

#### Do

```ts
const value = input;
```


### TSX

#### Don't

```tsx
const value = true && input;
```

#### Do

```tsx
const value = input;
```
