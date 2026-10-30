---
id: js/no-useless-continue-biome
title: "No continue at the end of a loop"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [biome/complexity/noUselessContinue]
since: 0.5.0
---

A loop body does not end with a bare `continue;`.

**Why:** The loop continues at the end of its body anyway; the statement is dead weight.

```codelaw
match:
  kind: continue_statement
  not: { has: { kind: statement_identifier } }
  nthChild: { position: 1, reverse: true }
  inside:
    kind: statement_block
    inside:
      any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}]
      field: body
message: "This `continue` is the loop body's last statement and does nothing"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
for (const x of xs) {
  log(x);
  continue;
}
```

#### Do

```js
for (const x of xs) {
  log(x);
}
```

#### Fixed

```js
for (const x of xs) {
  log(x);
}
```

### JSX

#### Don't

```jsx
for (const x of xs) {
  log(x);
  continue;
}
```

#### Do

```jsx
for (const x of xs) {
  log(x);
}
```

#### Fixed

```jsx
for (const x of xs) {
  log(x);
}
```

### TypeScript

#### Don't

```ts
for (const x of xs) {
  log(x);
  continue;
}
```

#### Do

```ts
for (const x of xs) {
  log(x);
}
```

#### Fixed

```ts
for (const x of xs) {
  log(x);
}
```

### TSX

#### Don't

```tsx
for (const x of xs) {
  log(x);
  continue;
}
```

#### Do

```tsx
for (const x of xs) {
  log(x);
}
```

#### Fixed

```tsx
for (const x of xs) {
  log(x);
}
```
