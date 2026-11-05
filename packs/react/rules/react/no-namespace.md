---
id: react/no-namespace
title: "No namespaced element names"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the element needs a real component or tag name"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-react/no-namespace]
since: 0.5.0
---

JSX element names have no namespace (`<svg:circle>`); React does not support them.

**Why:** React renders a namespaced name as an unknown element, and the namespace is lost.

```codelaw
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  has: { field: name, kind: jsx_namespace_name, pattern: $N }
message: "React doesn't support the namespaced name `$N`"
```

## How to fix

1. Use the plain tag (`<circle>` inside `<svg>`) or a component.

## Examples

### JavaScript

#### Don't

```js
const c = <svg:circle r={4} />;
```

#### Do

```js
const c = <circle r={4} />;
```

### JSX

#### Don't

```jsx
const c = <svg:circle r={4} />;
```

#### Do

```jsx
const c = <circle r={4} />;
```

### TSX

#### Don't

```tsx
const c = <svg:circle r={4} />;
```

#### Do

```tsx
const c = <circle r={4} />;
```
