---
id: js/curly
title: "Braces around every if, else and loop body"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/curly]
since: 0.5.0
---

The body of an `if`, `else`, `for`, `while` or `do` is a block in braces (the upstream default, `"all"`).

**Why:** Without braces, a second statement added under the first looks guarded and is not.

```codelaw

match:
  any:
    - kind: if_statement
      has: { field: consequence, not: { kind: statement_block }, pattern: $S }
    - kind: else_clause
      has: { not: { any: [{kind: statement_block}, {kind: if_statement}] }, pattern: $S }
    - any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}]
      has: { field: body, not: { kind: statement_block }, pattern: $S }
message: "Wrap the body in braces"
fix:
  - replace: { target: $S, with: "{ $S }" }
```

## Examples

### JavaScript

#### Don't

```js
if (done) return;
```

#### Do

```js
if (done) { return; }
```

### JSX

#### Don't

```jsx
if (done) return;
```

#### Do

```jsx
if (done) { return; }
```

### TypeScript

#### Don't

```ts
if (done) return;
```

#### Do

```ts
if (done) { return; }
```

### TSX

#### Don't

```tsx
if (done) return;
```

#### Do

```tsx
if (done) { return; }
```
