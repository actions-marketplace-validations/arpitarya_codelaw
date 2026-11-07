---
id: js/prefer-single-array-predicate
title: "One some() or every() call rather than two on the same array"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the combined callback runs both tests per element, interleaving their side effects; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-single-array-predicate]
since: 0.5.0
---

`a.some(x => p) || a.some(x => q)` walks the array twice; `a.some(x => p || q)` once. Likewise `a.every(x => p) && a.every(x => q)` is `a.every(x => p && q)`. The two callbacks name their parameter the same and return one expression each; different methods or arrays are left.

**Why:** One pass, and the two conditions sit together where they can be read as one test.

```codelaw
utils:
  arr:
    any:
      - kind: identifier
      - kind: member_expression
        not: { has: { stopBy: end, any: [{ kind: call_expression }, { kind: optional_chain }] } }
match:
  kind: binary_expression
  any:
    - all:
        - any:
            - pattern: "$A.some(($X) => $P) || $A.some(($X) => $Q)"
            - pattern: "$A.some($X => $P) || $A.some($X => $Q)"
        - pattern: $W1
    - all:
        - any:
            - pattern: "$A.every(($X) => $R) && $A.every(($X) => $S)"
            - pattern: "$A.every($X => $R) && $A.every($X => $S)"
        - pattern: $W2
constraints:
  A: { matches: arr }
message: "Test both conditions in one pass"
fix:
  - replace: { target: $W1, with: "$A.some(($X) => ($P) || ($Q))" }
  - replace: { target: $W2, with: "$A.every(($X) => ($R) && ($S))" }
```

## Examples

### JavaScript

#### Don't

```js
const bad = rows.some((r) => r.error) || rows.some((r) => r.warning);
```

#### Do

```js
const bad = rows.some((r) => (r.error) || (r.warning));
```


### JSX

#### Don't

```jsx
const bad = rows.some((r) => r.error) || rows.some((r) => r.warning);
```

#### Do

```jsx
const bad = rows.some((r) => (r.error) || (r.warning));
```


### TypeScript

#### Don't

```ts
const bad = rows.some((r) => r.error) || rows.some((r) => r.warning);
```

#### Do

```ts
const bad = rows.some((r) => (r.error) || (r.warning));
```


### TSX

#### Don't

```tsx
const bad = rows.some((r) => r.error) || rows.some((r) => r.warning);
```

#### Do

```tsx
const bad = rows.some((r) => (r.error) || (r.warning));
```
