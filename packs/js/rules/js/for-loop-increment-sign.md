---
id: js/for-loop-increment-sign
title: "A for loop counts towards its end"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the condition or the update is wrong is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/for-loop-increment-sign]
since: 0.5.0
---

A `for` loop's update moves the counter towards the bound its condition tests.

**Why:** Counting the wrong way makes the loop run forever or not at all.

```codelaw

match:
  kind: for_statement
  any:
    - all: [{ has: { field: condition, pattern: "$I < $N" } }, { has: { field: increment, any: [{ pattern: "$I--" }, { pattern: "--$I" }, { pattern: "$I -= $X" }] } }]
    - all: [{ has: { field: condition, pattern: "$I <= $N" } }, { has: { field: increment, any: [{ pattern: "$I--" }, { pattern: "--$I" }, { pattern: "$I -= $X" }] } }]
    - all: [{ has: { field: condition, pattern: "$I > $N" } }, { has: { field: increment, any: [{ pattern: "$I++" }, { pattern: "++$I" }, { pattern: "$I += $X" }] } }]
    - all: [{ has: { field: condition, pattern: "$I >= $N" } }, { has: { field: increment, any: [{ pattern: "$I++" }, { pattern: "++$I" }, { pattern: "$I += $X" }] } }]
message: "The counter moves away from the end condition; the loop never ends or never runs"
```

## How to fix

1. Flip the update (`i++`/`i--`) or the comparison so they agree.

## Examples

### JavaScript

#### Don't

```js
for (let i = 0; i < n; i--) {
  use(i);
}
```

#### Do

```js
for (let i = 0; i < n; i++) {
  use(i);
}
```

### JSX

#### Don't

```jsx
for (let i = 0; i < n; i--) {
  use(i);
}
```

#### Do

```jsx
for (let i = 0; i < n; i++) {
  use(i);
}
```

### TypeScript

#### Don't

```ts
for (let i = 0; i < n; i--) {
  use(i);
}
```

#### Do

```ts
for (let i = 0; i < n; i++) {
  use(i);
}
```

### TSX

#### Don't

```tsx
for (let i = 0; i < n; i--) {
  use(i);
}
```

#### Do

```tsx
for (let i = 0; i < n; i++) {
  use(i);
}
```
