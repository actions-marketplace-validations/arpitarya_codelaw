---
id: js/no-negated-array-predicate
title: "every() rather than a negated some(), and the reverse"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-negated-array-predicate]
since: 0.5.0
---

`!array.some(x => test)` is `array.every(x => !test)`, and `!array.every(x => test)` is `array.some(x => !test)` — De Morgan's law on a callback that returns one expression. A callback that already negates its test drops the negation instead.

**Why:** The negation sits next to the condition it negates, not in front of the whole call.

```codelaw
utils:
  plain:
    any:
      - kind: call_expression
      - kind: identifier
      - kind: member_expression
      - kind: subscript_expression
      - kind: parenthesized_expression
match:
  kind: unary_expression
  any:
    - all:
        - any:
            - pattern: "!$A.$M(($X) => $E)"
            - pattern: "!$A.$M($X => $E)"
        - has: { stopBy: end, kind: arrow_function, has: { field: body, matches: plain } }
        - pattern: $W1
    - all:
        - pattern: "!$A.$M(($X) => !$F)"
        - pattern: $W2
    - all:
        - pattern: "!$A.$M($X => !$F)"
        - pattern: $W2
constraints:
  M: { regex: '^(some|every)$' }
message: "Move the negation into the callback"
fix:
  - replace: { target: $W1, with: "$A.$M|map(some:every,every:some)(($X) => !$E)" }
  - replace: { target: $W2, with: "$A.$M|map(some:every,every:some)(($X) => $F)" }
```

## Examples

### JavaScript

#### Don't

```js
const missing = !users.some((user) => user.admin);
```

#### Do

```js
const missing = users.every((user) => !user.admin);
```


### JSX

#### Don't

```jsx
const missing = !users.some((user) => user.admin);
```

#### Do

```jsx
const missing = users.every((user) => !user.admin);
```


### TypeScript

#### Don't

```ts
const missing = !users.some((user) => user.admin);
```

#### Do

```ts
const missing = users.every((user) => !user.admin);
```


### TSX

#### Don't

```tsx
const missing = !users.some((user) => user.admin);
```

#### Do

```tsx
const missing = users.every((user) => !user.admin);
```
