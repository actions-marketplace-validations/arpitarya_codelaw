---
id: js/no-label-var
title: "No label with a variable's name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new label name is the author's choice"
stability: preview
ported_from: [eslint/no-label-var]
since: 0.5.0
---

A label does not reuse the name of a variable declared in the file.

**Why:** Labels and variables live in different namespaces, so the shared name reads as one thing when it is two.

```codelaw

match:
  kind: labeled_statement
  has: { field: label, pattern: $L }
  inside: { stopBy: end, kind: program, has: { stopBy: end, kind: variable_declarator, has: { field: name, pattern: $L } } }
message: "The label `$L` has the name of a variable; rename it"
```

## How to fix

1. Rename the label and every `break`/`continue` that names it.

## Examples

### JavaScript

#### Don't

```js
const loop = 1;
loop: for (;;) {
  break loop;
}
```

#### Do

```js
const loop = 1;
scan: for (;;) {
  break scan;
}
```

### JSX

#### Don't

```jsx
const loop = 1;
loop: for (;;) {
  break loop;
}
```

#### Do

```jsx
const loop = 1;
scan: for (;;) {
  break scan;
}
```

### TypeScript

#### Don't

```ts
const loop = 1;
loop: for (;;) {
  break loop;
}
```

#### Do

```ts
const loop = 1;
scan: for (;;) {
  break scan;
}
```

### TSX

#### Don't

```tsx
const loop = 1;
loop: for (;;) {
  break loop;
}
```

#### Do

```tsx
const loop = 1;
scan: for (;;) {
  break scan;
}
```
