---
id: js/no-unmodified-loop-condition
title: "No loop condition the loop never changes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the loop either misses an update or is meant to run differently, which the author decides"
fix_exception: restructure
stability: preview
ported_from: [eslint/no-unmodified-loop-condition]
since: 0.5.0
---

A variable in a `while`, `do…while` or `for` condition that nothing in the loop assigns is reported: the condition can't change, so the loop runs never or forever. In a comparison or `?:` (a group), the condition is reported only when no variable of the group changes. A condition that calls something, reads a property, constructs, awaits or yields is not judged, and a write in a function declaration the loop calls counts as a change.

**Why:** An unchanged condition is almost always a missing increment or the wrong variable.

```codelaw
native: js/no-unmodified-loop-condition
```

## How to fix

1. Update the variable in the loop body, or test the variable the loop does change.

## Examples

### JavaScript

#### Don't

```js
let node = head;
while (node) {
  visit(head);
}
```

#### Do

```js
let node = head;
while (node) {
  visit(node);
  node = node.next;
}
```


### JSX

#### Don't

```jsx
let node = head;
while (node) {
  visit(head);
}
```

#### Do

```jsx
let node = head;
while (node) {
  visit(node);
  node = node.next;
}
```


### TypeScript

#### Don't

```ts
let node = head;
while (node) {
  visit(head);
}
```

#### Do

```ts
let node = head;
while (node) {
  visit(node);
  node = node.next;
}
```


### TSX

#### Don't

```tsx
let node = head;
while (node) {
  visit(head);
}
```

#### Do

```tsx
let node = head;
while (node) {
  visit(node);
  node = node.next;
}
```
