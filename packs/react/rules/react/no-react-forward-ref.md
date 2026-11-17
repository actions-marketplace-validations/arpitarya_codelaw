---
id: react/no-react-forward-ref
title: "No `forwardRef`"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the `ref` parameter moves into the props, and the component's callers and types change with it"
fix_exception: restructure
stability: preview
ported_from: [biome/suspicious/noReactForwardRef, "@eslint-react/no-forward-ref"]
since: 0.5.1
---

`forwardRef(…)` and `React.forwardRef(…)` imported from `react` are not called: from React 19 a component takes `ref` as a prop. Behaviour of Biome's `noReactForwardRef`.

**Why:** `forwardRef` is deprecated in React 19; a plain function component with a `ref` prop does the same job and shows up in the props type.

```codelaw
match:
  kind: call_expression
  has: { field: function, pattern: $FN }
constraints:
  FN: { regex: '^(forwardRef|React\.forwardRef)$' }
where:
  $FN: { imported-from: react }
message: "Take `ref` as a prop instead of calling `forwardRef`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
import { forwardRef } from "react";

export const Input = forwardRef((props, ref) => <input ref={ref} {...props} />);
```

#### Do

```js
export function Input({ ref, ...props }) {
  return <input ref={ref} {...props} />;
}
```

### JSX

#### Don't

```jsx
import { forwardRef } from "react";

export const Input = forwardRef((props, ref) => <input ref={ref} {...props} />);
```

#### Do

```jsx
export function Input({ ref, ...props }) {
  return <input ref={ref} {...props} />;
}
```

### TSX

#### Don't

```tsx
import { forwardRef } from "react";

export const Input = forwardRef((props, ref) => <input ref={ref} {...props} />);
```

#### Do

```tsx
export function Input({ ref, ...props }) {
  return <input ref={ref} {...props} />;
}
```
