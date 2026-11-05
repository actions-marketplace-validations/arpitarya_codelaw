---
id: react/no-react-children
title: "No `React.Children` utilities"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "passing the data as props changes the component's API"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react/no-react-children]
since: 0.5.0
---

`React.Children.map`, `.forEach`, `.count`, `.only` and `.toArray` aren't called.

**Why:** They read children as data, which fragments and wrappers break.

```codelaw
native: react/no-react-children
```

## How to fix

1. Take the items as a prop and render them.

## Examples

### JavaScript

#### Don't

```js
const n = React.Children.count(children);
```

#### Do

```js
const n = items.length;
```

### JSX

#### Don't

```jsx
const n = React.Children.count(children);
```

#### Do

```jsx
const n = items.length;
```

### TSX

#### Don't

```tsx
const n = React.Children.count(children);
```

#### Do

```tsx
const n = items.length;
```
