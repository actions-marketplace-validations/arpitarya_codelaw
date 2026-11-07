---
id: js/prefer-else-if
title: "else if between adjacent ifs testing one value against different literals"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`else` skips the second test once the first matched; that is the same only when the first branch leaves the value alone and the two literals differ, so the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-else-if]
since: 0.5.0
---

Two `if` statements in a row that compare the same identifier with `===` against literals (`if (x === 1) …` then `if (x === 2) …`), the first without an `else` and not leaving the block (no `return`, `throw`, `break` or `continue`), are one `if … else if`.

**Why:** The chain says the cases exclude each other and stops testing after a match.

```codelaw
match:
  kind: if_statement
  pattern: "if ($X === $B) $S"
  follows:
    kind: if_statement
    pattern: "if ($X === $A) $T"
    not:
      any:
        - has: { kind: else_clause }
        - has: { stopBy: end, any: [{ kind: return_statement }, { kind: throw_statement }, { kind: break_statement }, { kind: continue_statement }] }
  not: { inside: { kind: else_clause } }
constraints:
  X: { kind: identifier }
  A: { any: [{ kind: string }, { kind: number }] }
  B: { any: [{ kind: string }, { kind: number }] }
message: "Join this test to the one before with `else if`"
fix:
  - insert-before: { text: "else " }
```

## Examples

### JavaScript

#### Don't

```js
if (mode === "read") {
  open();
}
if (mode === "write") {
  create();
}
```

#### Do

```js
if (mode === "read") {
  open();
} else if (mode === "write") {
  create();
}
```

#### Fixed

```js
if (mode === "read") {
  open();
}
else if (mode === "write") {
  create();
}
```


### JSX

#### Don't

```jsx
if (mode === "read") {
  open();
}
if (mode === "write") {
  create();
}
```

#### Do

```jsx
if (mode === "read") {
  open();
} else if (mode === "write") {
  create();
}
```

#### Fixed

```jsx
if (mode === "read") {
  open();
}
else if (mode === "write") {
  create();
}
```


### TypeScript

#### Don't

```ts
if (mode === "read") {
  open();
}
if (mode === "write") {
  create();
}
```

#### Do

```ts
if (mode === "read") {
  open();
} else if (mode === "write") {
  create();
}
```

#### Fixed

```ts
if (mode === "read") {
  open();
}
else if (mode === "write") {
  create();
}
```


### TSX

#### Don't

```tsx
if (mode === "read") {
  open();
}
if (mode === "write") {
  create();
}
```

#### Do

```tsx
if (mode === "read") {
  open();
} else if (mode === "write") {
  create();
}
```

#### Fixed

```tsx
if (mode === "read") {
  open();
}
else if (mode === "write") {
  create();
}
```
