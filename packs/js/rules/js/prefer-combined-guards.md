---
id: js/prefer-combined-guards
title: "One guard for two that exit the same way"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-combined-guards]
since: 0.5.0
---

Two `if` statements in a row that each only `return;` (or only `continue;`) are one `if` with their conditions joined by `||`.

**Why:** The two guards do the same thing; one says so in a line.

```codelaw
utils:
  operand:
    not:
      any:
        - kind: ternary_expression
        - kind: assignment_expression
        - kind: sequence_expression
        - kind: arrow_function
        - kind: yield_expression
        - kind: binary_expression
          has: { field: operator, regex: '^\?\?$' }
match:
  any:
    - pattern: "if ($A) return;"
      not: { has: { kind: else_clause } }
      precedes: { pattern: "if ($B) return;", not: { has: { kind: else_clause } }, all: [{ pattern: $SECOND }] }
      all: [{ pattern: $RET }]
    - pattern: "if ($A) continue;"
      not: { has: { kind: else_clause } }
      precedes: { pattern: "if ($B) continue;", not: { has: { kind: else_clause } }, all: [{ pattern: $SECOND }] }
      all: [{ pattern: $CONT }]
constraints:
  A: { matches: operand }
  B: { matches: operand }
message: "Combine the two guards"
fix:
  - replace: { target: $RET, with: "if ($A || $B) return;" }
  - replace: { target: $CONT, with: "if ($A || $B) continue;" }
  - remove: $SECOND
```

## Examples

### JavaScript

#### Don't

```js
function save(user) {
  if (!user) return;
  if (user.locked) return;
  store(user);
}
```

#### Do

```js
function save(user) {
  if (!user || user.locked) return;
  store(user);
}
```


### JSX

#### Don't

```jsx
function save(user) {
  if (!user) return;
  if (user.locked) return;
  store(user);
}
```

#### Do

```jsx
function save(user) {
  if (!user || user.locked) return;
  store(user);
}
```


### TypeScript

#### Don't

```ts
function save(user) {
  if (!user) return;
  if (user.locked) return;
  store(user);
}
```

#### Do

```ts
function save(user) {
  if (!user || user.locked) return;
  store(user);
}
```


### TSX

#### Don't

```tsx
function save(user) {
  if (!user) return;
  if (user.locked) return;
  store(user);
}
```

#### Do

```tsx
function save(user) {
  if (!user || user.locked) return;
  store(user);
}
```
