---
id: js/no-continue
title: "No continue"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "inverting the condition restructures the loop body"
stability: preview
ported_from: [eslint/no-continue]
since: 0.5.0
---

Loops do not use `continue`.

**Why:** A `continue` jumps over the rest of the loop body; an `if` around it states the condition where it applies.

```codelaw
match:
  kind: continue_statement
message: "Unexpected `continue`"
```

## How to fix

1. Invert the condition and wrap the rest of the body in an `if`.

## Examples

### JavaScript

#### Don't

```js
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Do

```js
for (const x of xs) {
  if (x) use(x);
}
```

### JSX

#### Don't

```jsx
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Do

```jsx
for (const x of xs) {
  if (x) use(x);
}
```

### TypeScript

#### Don't

```ts
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Do

```ts
for (const x of xs) {
  if (x) use(x);
}
```

### TSX

#### Don't

```tsx
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Do

```tsx
for (const x of xs) {
  if (x) use(x);
}
```
