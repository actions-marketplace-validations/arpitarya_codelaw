---
id: js/prefer-simple-sort-comparator
title: "A subtraction comparator rather than comparing and returning 1 or -1"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "subtraction orders numbers only; for strings the comparison form works and `a.localeCompare(b)` is the replacement, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-simple-sort-comparator]
since: 0.5.0
---

A `sort` or `toSorted` comparator that compares two values and returns `1` or `-1` (`(a, b) => a > b ? 1 : -1`, or the `if … return 1; if … return -1; return 0` chain) is `(a, b) => a - b` for numbers, or `(a, b) => b - a` for the reverse order.

**Why:** The subtraction is the standard comparator; the three-way form makes the reader trace which way it sorts.

```codelaw
match:
  kind: call_expression
  pattern: "$ARR.$SORT($CMP)"
  has:
    field: arguments
    has:
      kind: arrow_function
      any:
        - all:
            - any:
                - pattern: "($A, $B) => $A > $B ? 1 : -1"
                - pattern: "($A, $B) => $A < $B ? -1 : 1"
                - pattern: "($A, $B) => { if ($A > $B) { return 1; } if ($A < $B) { return -1; } return 0; }"
                - pattern: "($A, $B) => { if ($A > $B) return 1; if ($A < $B) return -1; return 0; }"
            - has: { field: parameters, has: { nthChild: 1, pattern: $X1 } }
            - has: { field: parameters, has: { nthChild: 2, pattern: $Y1 } }
            - pattern: $F1
        - all:
            - any:
                - pattern: "($A, $B) => $A > $B ? -1 : 1"
                - pattern: "($A, $B) => $A < $B ? 1 : -1"
                - pattern: "($A, $B) => { if ($A > $B) { return -1; } if ($A < $B) { return 1; } return 0; }"
                - pattern: "($A, $B) => { if ($A > $B) return -1; if ($A < $B) return 1; return 0; }"
            - has: { field: parameters, has: { nthChild: 1, pattern: $X2 } }
            - has: { field: parameters, has: { nthChild: 2, pattern: $Y2 } }
            - pattern: $F2
constraints:
  SORT: { regex: '^(sort|toSorted)$' }
message: "Compare with a subtraction"
fix:
  - replace: { target: $F1, with: "($X1, $Y1) => $X1 - $Y1" }
  - replace: { target: $F2, with: "($X2, $Y2) => $Y2 - $X2" }
```

## Examples

### JavaScript

#### Don't

```js
scores.sort((a, b) => a > b ? 1 : -1);
```

#### Do

```js
scores.sort((a, b) => a - b);
```


### JSX

#### Don't

```jsx
scores.sort((a, b) => a > b ? 1 : -1);
```

#### Do

```jsx
scores.sort((a, b) => a - b);
```


### TypeScript

#### Don't

```ts
scores.sort((a, b) => a > b ? 1 : -1);
```

#### Do

```ts
scores.sort((a, b) => a - b);
```


### TSX

#### Don't

```tsx
scores.sort((a, b) => a > b ? 1 : -1);
```

#### Do

```tsx
scores.sort((a, b) => a - b);
```
