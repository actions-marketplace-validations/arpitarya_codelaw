---
id: js/updated-loop-counter
title: "No assignment to a for loop counter in its body"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the rewrite (a `while` loop, a `continue`) depends on intent"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/updated-loop-counter]
since: 0.5.0
---

The variable a `for` loop declares as its counter is not assigned in the loop body.

**Why:** Changing the counter in two places makes the iteration hard to follow and easy to break.

```codelaw

match:
  any:
    - { kind: assignment_expression, has: { field: left, kind: identifier, pattern: $I } }
    - { kind: augmented_assignment_expression, has: { field: left, kind: identifier, pattern: $I } }
    - { kind: update_expression, has: { kind: identifier, pattern: $I } }
  inside:
    stopBy: end
    kind: statement_block
    inside: { kind: for_statement, field: body, has: { field: initializer, has: { kind: variable_declarator, has: { field: name, pattern: $I } } } }
message: "The loop counter `$I` is changed inside the loop body"
```

## How to fix

1. Move the change into the loop's update, use `continue`, or rewrite it as a `while` loop.

## Examples

### JavaScript

#### Don't

```js
for (let i = 0; i < n; i++) {
  if (skip(i)) i++;
}
```

#### Do

```js
for (let i = 0; i < n; i++) {
  if (skip(i)) continue;
}
```

### JSX

#### Don't

```jsx
for (let i = 0; i < n; i++) {
  if (skip(i)) i++;
}
```

#### Do

```jsx
for (let i = 0; i < n; i++) {
  if (skip(i)) continue;
}
```

### TypeScript

#### Don't

```ts
for (let i = 0; i < n; i++) {
  if (skip(i)) i++;
}
```

#### Do

```ts
for (let i = 0; i < n; i++) {
  if (skip(i)) continue;
}
```

### TSX

#### Don't

```tsx
for (let i = 0; i < n; i++) {
  if (skip(i)) i++;
}
```

#### Do

```tsx
for (let i = 0; i < n; i++) {
  if (skip(i)) continue;
}
```
