---
id: js/no-extra-label
title: "No label that is not needed"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "removing the label token from the statement is not a template edit"
fix_pending: "auto; needs computed-edit"
stability: preview
ported_from: [eslint/no-extra-label]
since: 0.5.0
---

A `break` or `continue` does not name a label that is the innermost loop or `switch` around it.

**Why:** The label says nothing the bare statement does not.

```codelaw

match:
  any: [{kind: break_statement}, {kind: continue_statement}]
  has: { kind: statement_identifier, pattern: $L }
  inside:
    stopBy: { any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}, {kind: switch_statement}] }
    any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}, {kind: switch_statement}]
    inside: { kind: labeled_statement, has: { field: label, pattern: $L } }
message: "`$L` is the innermost loop already; drop the label"
```

## How to fix

1. Delete the label after `break` or `continue`; delete the label itself if nothing else uses it.

## Examples

### JavaScript

#### Don't

```js
outer: for (const x of xs) {
  if (x) break outer;
}
```

#### Do

```js
outer: for (const x of xs) {
  if (x) break;
}
```

### JSX

#### Don't

```jsx
outer: for (const x of xs) {
  if (x) break outer;
}
```

#### Do

```jsx
outer: for (const x of xs) {
  if (x) break;
}
```

### TypeScript

#### Don't

```ts
outer: for (const x of xs) {
  if (x) break outer;
}
```

#### Do

```ts
outer: for (const x of xs) {
  if (x) break;
}
```

### TSX

#### Don't

```tsx
outer: for (const x of xs) {
  if (x) break outer;
}
```

#### Do

```tsx
outer: for (const x of xs) {
  if (x) break;
}
```
