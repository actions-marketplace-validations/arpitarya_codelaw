---
id: js/no-unexpected-multiline
title: "A line break doesn't silently join two lines into one expression"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the choices are a `;` ending the first line, splitting the two, or joining them, keeping what JavaScript parses today; only the author knows which was meant"
stability: preview
ported_from: [eslint/no-unexpected-multiline, eslint-plugin-sonarjs/call-argument-line]
since: 0.1.0
---

A line that starts with `(`, `[` or a template literal does not continue the expression on the line
before it without a semicolon.

**Why:** Without semicolons, JavaScript joins such lines: `a\n(b)` is the call `a(b)`, which is
rarely what was written.

```codelaw
native: js/no-unexpected-multiline
```

## How to fix

1. If the lines are separate statements, end the first with a semicolon.
2. If they are one expression, move the `(` or `[` onto the first line.

## Examples

### JavaScript

#### Don't

```js
const total = sum
(items).forEach(log);
```

#### Fixed (split)

```js
const total = sum;
(items).forEach(log);
```

#### Fixed (join)

```js
const total = sum(items).forEach(log);
```

#### Do

```js
const total = sum;
(items).forEach(log);
```

### JSX

#### Don't

```jsx
const total = sum
(items).forEach(log);
```

#### Fixed (split)

```jsx
const total = sum;
(items).forEach(log);
```

#### Fixed (join)

```jsx
const total = sum(items).forEach(log);
```

#### Do

```jsx
const total = sum;
(items).forEach(log);
```

### TypeScript

#### Don't

```ts
const total = sum
(items).forEach(log);
```

#### Fixed (split)

```ts
const total = sum;
(items).forEach(log);
```

#### Fixed (join)

```ts
const total = sum(items).forEach(log);
```

#### Do

```ts
const total = sum;
(items).forEach(log);
```

### TSX

#### Don't

```tsx
const total = sum
(items).forEach(log);
```

#### Fixed (split)

```tsx
const total = sum;
(items).forEach(log);
```

#### Fixed (join)

```tsx
const total = sum(items).forEach(log);
```

#### Do

```tsx
const total = sum;
(items).forEach(log);
```
