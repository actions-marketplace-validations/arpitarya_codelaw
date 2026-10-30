---
id: js/no-labels
title: "No labels"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "replacing a labelled jump means restructuring the loops, often into a function with a `return`"
stability: preview
ported_from: [eslint/no-labels]
since: 0.5.0
---

Statements are not labelled, and `break`/`continue` take no label.

**Why:** Labels are rare enough that readers misread them, and they are mostly a `goto` in disguise.

```codelaw
match:
  kind: labeled_statement
message: "Unexpected label; move the loop into a function and `return` instead"
```

## How to fix

1. Move the nested loops into a function and `return` from it.

## Examples

### JavaScript

#### Don't

```js
outer: for (const a of as) {
  for (const b of bs) {
    if (a === b) break outer;
  }
}
```

#### Do

```js
function find() {
  for (const a of as) {
    for (const b of bs) {
      if (a === b) return;
    }
  }
}
```

### JSX

#### Don't

```jsx
outer: for (const a of as) {
  for (const b of bs) {
    if (a === b) break outer;
  }
}
```

#### Do

```jsx
function find() {
  for (const a of as) {
    for (const b of bs) {
      if (a === b) return;
    }
  }
}
```

### TypeScript

#### Don't

```ts
outer: for (const a of as) {
  for (const b of bs) {
    if (a === b) break outer;
  }
}
```

#### Do

```ts
function find() {
  for (const a of as) {
    for (const b of bs) {
      if (a === b) return;
    }
  }
}
```

### TSX

#### Don't

```tsx
outer: for (const a of as) {
  for (const b of bs) {
    if (a === b) break outer;
  }
}
```

#### Do

```tsx
function find() {
  for (const a of as) {
    for (const b of bs) {
      if (a === b) return;
    }
  }
}
```
