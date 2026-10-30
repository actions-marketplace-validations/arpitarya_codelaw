---
id: react/forward-ref-uses-ref
title: "A forwardRef callback takes the ref"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "adding the `ref` parameter is what the wrapper is for, but the author still has to attach it, or drop `forwardRef` instead"
stability: preview
ported_from: [eslint-plugin-react/forward-ref-uses-ref]
since: 0.5.0
---

The function passed to `forwardRef` takes a second parameter, the ref.

**Why:** `forwardRef` around a function that ignores the ref forwards nothing; either use the ref or drop the wrapper.

```codelaw
match:
  any: [{ kind: arrow_function }, { kind: function_expression }]
  inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(React\.)?forwardRef$' } } }
  has:
    field: parameters
    kind: formal_parameters
    all:
      - has: { nthChild: 1, pattern: $P, not: { kind: rest_pattern } }
      - not: { has: { nthChild: 2 } }
message: "This `forwardRef` callback never takes the ref"
fix:
  - insert-after: { target: $P, text: ", ref" }
```

## Examples

### JavaScript

#### Don't

```js
const Input = forwardRef((props) => <input {...props} />);
```

#### Do

```js
const Input = forwardRef((props, ref) => <input ref={ref} {...props} />);
```

#### Fixed

```js
const Input = forwardRef((props, ref) => <input {...props} />);
```

### JSX

#### Don't

```jsx
const Input = forwardRef((props) => <input {...props} />);
```

#### Do

```jsx
const Input = forwardRef((props, ref) => <input ref={ref} {...props} />);
```

#### Fixed

```jsx
const Input = forwardRef((props, ref) => <input {...props} />);
```

### TSX

#### Don't

```tsx
const Input = forwardRef((props) => <input {...props} />);
```

#### Do

```tsx
const Input = forwardRef((props, ref) => <input ref={ref} {...props} />);
```

#### Fixed

```tsx
const Input = forwardRef((props, ref) => <input {...props} />);
```
