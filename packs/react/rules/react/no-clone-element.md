---
id: react/no-clone-element
title: "No `cloneElement`"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "replacing a clone with props or a render function changes the component's API"
fix_exception: restructure
stability: preview
ported_from: [oxlint/react/no-clone-element]
since: 0.5.0
---

`React.cloneElement` isn't called.

**Why:** Cloning hides which props a child really gets, and breaks when the child is wrapped.

```codelaw
native: react/no-clone-element
```

## How to fix

1. Pass the props explicitly, or take a render function.

## Examples

### JavaScript

#### Don't

```js
const b = React.cloneElement(child, { size: 2 });
```

#### Do

```js
const b = <Button size={2} />;
```

### JSX

#### Don't

```jsx
const b = React.cloneElement(child, { size: 2 });
```

#### Do

```jsx
const b = <Button size={2} />;
```

### TSX

#### Don't

```tsx
const b = React.cloneElement(child, { size: 2 });
```

#### Do

```tsx
const b = <Button size={2} />;
```
