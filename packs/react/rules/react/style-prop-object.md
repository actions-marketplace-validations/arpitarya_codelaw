---
id: react/style-prop-object
title: "style is an object"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "each value stays a string, where React would also take a number for a length; a value with quotes or `!important`, or a `style` that is not a plain string, is rewritten by hand"
stability: preview
ported_from: [eslint-plugin-react/style-prop-object]
since: 0.5.0
---

The `style` prop is an object (`style={{ color: "red" }}`), not a string, number or boolean.

**Why:** React throws on a string `style`; it expects a map of properties.

```codelaw
match:
  kind: jsx_attribute
  regex: '^style\b'
  all:
    - any:
        - has: { kind: string }
        - has: { kind: jsx_expression, has: { any: [{ kind: string }, { kind: template_string }, { kind: number }, { kind: "true" }, { kind: "false" }] } }
    - any:
        - has: { kind: string, pattern: $CSS }
        - regex: ''
message: "`style` takes an object, not a string"
fix:
  - replace: { target: $CSS, with: "{$CSS|style-object}" }
```

## How to fix

1. Rewrite the CSS as an object: `style={{ color: "red", marginTop: 4 }}`.

## Examples

### JavaScript

#### Don't

```js
const d = <div style="color: red" />;
```

#### Do

```js
const d = <div style={{ color: "red" }} />;
```

### JSX

#### Don't

```jsx
const d = <div style="color: red" />;
```

#### Do

```jsx
const d = <div style={{ color: "red" }} />;
```

### TSX

#### Don't

```tsx
const d = <div style="color: red" />;
```

#### Do

```tsx
const d = <div style={{ color: "red" }} />;
```
