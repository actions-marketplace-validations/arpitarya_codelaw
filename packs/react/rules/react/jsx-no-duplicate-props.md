---
id: react/jsx-no-duplicate-props
title: "No prop given twice on one element"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "the earlier of two props never takes effect, since the last one wins; it goes, automatically, when its value is a literal or a name, which nothing evaluates to any effect. An earlier value that calls something is manual"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/jsx-no-duplicate-props]
since: 0.1.0
---

A JSX element gives each prop once.

**Why:** The last value silently wins, so the first is dead code or a mistake.

```codelaw
native: react/jsx-no-duplicate-props
```

## How to fix

1. Keep the value that is right and remove the other.

## Examples

### JavaScript

#### Don't

```js
const a = <input type="text" type="email" />;
```

#### Do

```js
const a = <input type="email" />;
```

### JSX

#### Don't

```jsx
const a = <input type="text" type="email" />;
```

#### Do

```jsx
const a = <input type="email" />;
```

### TSX

#### Don't

```tsx
const a = <input type="text" type="email" />;
```

#### Do

```tsx
const a = <input type="email" />;
```
