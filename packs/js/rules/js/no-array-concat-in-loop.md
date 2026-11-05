---
id: js/no-array-concat-in-loop
title: "No concat() in a loop"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "`push` mutates the array in place, which other holders of the old array would see"
stability: preview
ported_from: [eslint-plugin-unicorn/no-array-concat-in-loop]
since: 0.5.0
---

An array is not rebuilt with `a = a.concat(...)` inside a loop.

**Why:** Each `concat` copies the whole array, so the loop is quadratic.

```codelaw

match:
  kind: assignment_expression
  pattern: $A = $A.concat($$$X)
  inside: { stopBy: end, any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}] }
message: "`concat` in a loop copies `$A` every time; push into it, or collect and flatten once"
```

## How to fix

1. Use `a.push(...items)`, or build a list of parts in the loop and `.flat()` it once after.

## Examples

### JavaScript

#### Don't

```js
for (const page of pages) {
  rows = rows.concat(page.rows);
}
```

#### Do

```js
for (const page of pages) {
  rows.push(...page.rows);
}
```

### JSX

#### Don't

```jsx
for (const page of pages) {
  rows = rows.concat(page.rows);
}
```

#### Do

```jsx
for (const page of pages) {
  rows.push(...page.rows);
}
```

### TypeScript

#### Don't

```ts
for (const page of pages) {
  rows = rows.concat(page.rows);
}
```

#### Do

```ts
for (const page of pages) {
  rows.push(...page.rows);
}
```

### TSX

#### Don't

```tsx
for (const page of pages) {
  rows = rows.concat(page.rows);
}
```

#### Do

```tsx
for (const page of pages) {
  rows.push(...page.rows);
}
```
