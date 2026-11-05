---
id: react/no-adjacent-inline-elements
title: "No inline elements that touch"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a space between the elements changes what is rendered"
stability: preview
ported_from: [eslint-plugin-react/no-adjacent-inline-elements]
since: 0.5.0
---

Two inline elements (`<a>`, `<span>`, `<b>`, …) that are siblings have something between them.

**Why:** With nothing between them, the rendered words run together.

```codelaw
native: react/no-adjacent-inline-elements
```

## How to fix

1. Add the space or text that was meant between them.

## Examples

### JavaScript

#### Don't

```js
const e = <div><a>x</a><a>y</a></div>;
```

#### Do

```js
const e = <div><a>x</a> <a>y</a></div>;
```

### JSX

#### Don't

```jsx
const e = <div><a>x</a><a>y</a></div>;
```

#### Do

```jsx
const e = <div><a>x</a> <a>y</a></div>;
```

### TSX

#### Don't

```tsx
const e = <div><a>x</a><a>y</a></div>;
```

#### Do

```tsx
const e = <div><a>x</a> <a>y</a></div>;
```
