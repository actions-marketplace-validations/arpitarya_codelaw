---
id: js/no-loss-of-precision
title: "Number literals keep their value"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the choices are the literal the engine already stores (behaviour-preserving) or, for an integer, a BigInt, which keeps every digit but is another type"
fix_exception: value
stability: preview
ported_from: [eslint/no-loss-of-precision, eslint-plugin-unicorn/no-loss-of-precision]
since: 0.1.0
---

A number literal can be represented exactly as a JavaScript number.

**Why:** A literal with more precision than a double holds is silently rounded, so the code doesn't
mean what it says.

```codelaw
native: js/no-loss-of-precision
```

## How to fix

1. Use a BigInt (`123n`) for an exact large integer.
2. Or keep the value as a string, or write the value the code really gets.

## Examples

### JavaScript

#### Don't

```js
const id = 9007199254740993;
```

#### Fixed (the value it holds)

```js
const id = 9007199254740992;
```

#### Fixed (BigInt)

```js
const id = 9007199254740993n;
```

#### Do

```js
const id = 9007199254740993n;
```

### JSX

#### Don't

```jsx
const id = 9007199254740993;
```

#### Fixed (the value it holds)

```jsx
const id = 9007199254740992;
```

#### Fixed (BigInt)

```jsx
const id = 9007199254740993n;
```

#### Do

```jsx
const id = 9007199254740993n;
```

### TypeScript

#### Don't

```ts
const id = 9007199254740993;
```

#### Fixed (the value it holds)

```ts
const id = 9007199254740992;
```

#### Fixed (BigInt)

```ts
const id = 9007199254740993n;
```

#### Do

```ts
const id = 9007199254740993n;
```

### TSX

#### Don't

```tsx
const id = 9007199254740993;
```

#### Fixed (the value it holds)

```tsx
const id = 9007199254740992;
```

#### Fixed (BigInt)

```tsx
const id = 9007199254740993n;
```

#### Do

```tsx
const id = 9007199254740993n;
```
