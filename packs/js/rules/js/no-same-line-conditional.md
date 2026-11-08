---
id: js/no-same-line-conditional
title: "No `if` starting on the line the previous `if` ends"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "whether the second `if` was meant as an `else if` or as its own statement is the author's intent"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-same-line-conditional]
since: 0.5.0
---

Two `if` statements in a row, where the second starts on the line the first ends (`} if (b) {`) and they don't both sit on one line, are reported at the second `if`. The suggestions are `else if`, or the second `if` on a line of its own.

**Why:** It reads like `else if`, but both conditions are tested and both branches can run.

```codelaw
native: js/no-same-line-conditional
```

## Examples

### JavaScript

#### Don't

```js
if (a) {
  x();
} if (b) {
  y();
}
```

#### Do

```js
if (a) {
  x();
} else if (b) {
  y();
}
```

#### Fixed

```js
if (a) {
  x();
} else if (b) {
  y();
}
```


### JSX

#### Don't

```jsx
if (a) {
  x();
} if (b) {
  y();
}
```

#### Do

```jsx
if (a) {
  x();
} else if (b) {
  y();
}
```

#### Fixed

```jsx
if (a) {
  x();
} else if (b) {
  y();
}
```


### TypeScript

#### Don't

```ts
if (a) {
  x();
} if (b) {
  y();
}
```

#### Do

```ts
if (a) {
  x();
} else if (b) {
  y();
}
```

#### Fixed

```ts
if (a) {
  x();
} else if (b) {
  y();
}
```


### TSX

#### Don't

```tsx
if (a) {
  x();
} if (b) {
  y();
}
```

#### Do

```tsx
if (a) {
  x();
} else if (b) {
  y();
}
```

#### Fixed

```tsx
if (a) {
  x();
} else if (b) {
  y();
}
```
