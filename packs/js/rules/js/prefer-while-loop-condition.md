---
id: js/prefer-while-loop-condition
title: "The loop condition in the while, not a break at the top"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-while-loop-condition]
since: 0.5.0
---

An endless loop (`while (true)`, `for (;;)`) whose body starts by breaking out on a condition puts that condition in the `while`.

**Why:** The loop's exit is then where a reader looks for it.

```codelaw
match:
  any:
    - pattern: "while (true) { if (!$C) break; $$$B }"
      all: [{ pattern: $NEG }]
    - pattern: "for (;;) { if (!$C) break; $$$B }"
      all: [{ pattern: $NEG }]
    - pattern: "while (true) { if ($D) break; $$$B }"
      not: { pattern: "while (true) { if (!$C) break; $$$B }" }
      all: [{ pattern: $POS }]
    - pattern: "for (;;) { if ($D) break; $$$B }"
      not: { pattern: "for (;;) { if (!$C) break; $$$B }" }
      all: [{ pattern: $POS }]
message: "Move the condition into the `while`"
fix:
  - replace: { target: $NEG, with: "while ($C) {\n  $$$B\n}" }
  - replace: { target: $POS, with: "while (!($D)) {\n  $$$B\n}" }
```

## Examples

### JavaScript

#### Don't

```js
while (true) {
  if (!queue.length) break;
  handle(queue.shift());
}
```

#### Do

```js
while (queue.length) {
  handle(queue.shift());
}
```


### JSX

#### Don't

```jsx
while (true) {
  if (!queue.length) break;
  handle(queue.shift());
}
```

#### Do

```jsx
while (queue.length) {
  handle(queue.shift());
}
```


### TypeScript

#### Don't

```ts
while (true) {
  if (!queue.length) break;
  handle(queue.shift());
}
```

#### Do

```ts
while (queue.length) {
  handle(queue.shift());
}
```


### TSX

#### Don't

```tsx
while (true) {
  if (!queue.length) break;
  handle(queue.shift());
}
```

#### Do

```tsx
while (queue.length) {
  handle(queue.shift());
}
```
