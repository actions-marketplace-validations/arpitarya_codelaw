---
id: js/label-position
title: "Only loops and switches are labelled"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the label was for, and how to write it without one, is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/label-position]
since: 0.5.0
---

A label is put only on a loop or a `switch`, where `break` and `continue` can use it.

**Why:** A label on any other statement is almost always a leftover, or an object literal's key mistaken for a statement.

```codelaw
match:
  kind: labeled_statement
  not:
    has:
      field: body
      any:
        - kind: for_statement
        - kind: for_in_statement
        - kind: while_statement
        - kind: do_statement
        - kind: switch_statement
message: "Only loops and switches take a label"
```

## How to fix

1. Remove the label, or move it onto the loop it was meant to name.

## Examples

### JavaScript

#### Don't

```js
found: {
  if (ready) break found;
  run();
}
```

#### Do

```js
outer: for (const row of rows) {
  for (const cell of row) {
    if (cell) continue outer;
  }
}
```


### JSX

#### Don't

```jsx
found: {
  if (ready) break found;
  run();
}
```

#### Do

```jsx
outer: for (const row of rows) {
  for (const cell of row) {
    if (cell) continue outer;
  }
}
```


### TypeScript

#### Don't

```ts
found: {
  if (ready) break found;
  run();
}
```

#### Do

```ts
outer: for (const row of rows) {
  for (const cell of row) {
    if (cell) continue outer;
  }
}
```


### TSX

#### Don't

```tsx
found: {
  if (ready) break found;
  run();
}
```

#### Do

```tsx
outer: for (const row of rows) {
  for (const cell of row) {
    if (cell) continue outer;
  }
}
```
