---
id: react/jsx-sort-props
title: "JSX props are in alphabetical order"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "reordering props reorders the evaluation of their values, and a later duplicate prop wins, so the order can matter"
stability: preview
options:
  ignore_case: { type: "boolean", default: false, description: "compare names without case" }
ported_from: [eslint-plugin-react/jsx-sort-props]
since: 0.5.0
---

An element's props are in alphabetical order; a spread starts a new run.

**Why:** A sorted list is easy to scan and to diff.

```codelaw
native: react/jsx-sort-props
```

## How to fix

1. Sort the props.

## Examples

### JavaScript

#### Don't

```js
const e = <a title="t" href="/" />;
```

#### Do

```js
const e = <a href="/" title="t" />;
```

### JSX

#### Don't

```jsx
const e = <a title="t" href="/" />;
```

#### Do

```jsx
const e = <a href="/" title="t" />;
```

### TSX

#### Don't

```tsx
const e = <a title="t" href="/" />;
```

#### Do

```tsx
const e = <a href="/" title="t" />;
```
