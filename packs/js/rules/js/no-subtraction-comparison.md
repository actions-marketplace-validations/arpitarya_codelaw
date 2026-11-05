---
id: js/no-subtraction-comparison
title: "a > b rather than a - b > 0"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "for non-numbers the subtraction converts and the comparison does not"
stability: preview
ported_from: [eslint-plugin-unicorn/no-subtraction-comparison]
since: 0.5.0
---

`a - b > 0` is written `a > b`. This port covers `> 0`.

**Why:** The direct comparison says the same and avoids overflow and NaN surprises.

```codelaw
match:
  pattern: "$A - $B > 0"
message: "Compare directly"
fix: "$A > $B"
```

## Examples

### JavaScript

#### Don't

```js
if (a - b > 0) {
  swap();
}
```

#### Do

```js
if (a > b) {
  swap();
}
```

### JSX

#### Don't

```jsx
if (a - b > 0) {
  swap();
}
```

#### Do

```jsx
if (a > b) {
  swap();
}
```

### TypeScript

#### Don't

```ts
if (a - b > 0) {
  swap();
}
```

#### Do

```ts
if (a > b) {
  swap();
}
```

### TSX

#### Don't

```tsx
if (a - b > 0) {
  swap();
}
```

#### Do

```tsx
if (a > b) {
  swap();
}
```
