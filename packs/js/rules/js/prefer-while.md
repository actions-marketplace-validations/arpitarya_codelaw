---
id: js/prefer-while
title: "while rather than for(;;) with only a condition"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-while]
since: 0.5.0
---

A `for` loop with no initialiser and no update is written as `while`.

**Why:** `while` says the same with less punctuation.

```codelaw

match:
  kind: for_statement
  pattern: "for (; $C; ) $B"
message: "A `for` with only a condition is a `while`"
fix: "while ($C) $B"
```

## Examples

### JavaScript

#### Don't

```js
for (; queue.length > 0; ) {
  run(queue.shift());
}
```

#### Do

```js
while (queue.length > 0) {
  run(queue.shift());
}
```

### JSX

#### Don't

```jsx
for (; queue.length > 0; ) {
  run(queue.shift());
}
```

#### Do

```jsx
while (queue.length > 0) {
  run(queue.shift());
}
```

### TypeScript

#### Don't

```ts
for (; queue.length > 0; ) {
  run(queue.shift());
}
```

#### Do

```ts
while (queue.length > 0) {
  run(queue.shift());
}
```

### TSX

#### Don't

```tsx
for (; queue.length > 0; ) {
  run(queue.shift());
}
```

#### Do

```tsx
while (queue.length > 0) {
  run(queue.shift());
}
```
