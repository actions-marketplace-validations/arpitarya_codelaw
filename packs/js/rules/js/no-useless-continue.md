---
id: js/no-useless-continue
title: "No continue at the end of a loop"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-continue]
since: 0.5.0
---

A `continue` is not the last statement of a loop body.

**Why:** The loop continues there anyway.

```codelaw
match:
  kind: continue_statement
  not: { has: { kind: statement_identifier } }
  nthChild: { position: 1, reverse: true }
  inside: { kind: statement_block, inside: { any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}], field: body } }
message: "The loop continues here anyway; remove `continue`"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
for (const x of xs) {
  use(x);
  continue;
}
```

#### Do

```js
for (const x of xs) {
  use(x);
}
```

### JSX

#### Don't

```jsx
for (const x of xs) {
  use(x);
  continue;
}
```

#### Do

```jsx
for (const x of xs) {
  use(x);
}
```

### TypeScript

#### Don't

```ts
for (const x of xs) {
  use(x);
  continue;
}
```

#### Do

```ts
for (const x of xs) {
  use(x);
}
```

### TSX

#### Don't

```tsx
for (const x of xs) {
  use(x);
  continue;
}
```

#### Do

```tsx
for (const x of xs) {
  use(x);
}
```
