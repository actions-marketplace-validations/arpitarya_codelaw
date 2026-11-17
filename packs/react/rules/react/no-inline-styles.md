---
id: react/no-inline-styles
title: "No object literal in a `style` prop"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the declarations move to a stylesheet, a class or a styled component the author names"
fix_exception: restructure
stability: preview
ported_from: [biome/style/noInlineStyles, eslint-plugin-react-native/no-inline-styles]
since: 0.5.1
---

A JSX element does not take a `style={{ … }}` object literal. A `style` set to a variable, a call or a member is left alone, since it is shared. Behaviour of Biome's `noInlineStyles` for the JSX form; `React.createElement` is not read.

**Why:** An inline style builds a new object on every render, cannot use media queries, pseudo-classes or a theme, and sits outside the project's stylesheets.

```codelaw
match:
  kind: jsx_attribute
  all:
    - has: { kind: property_identifier, regex: '^style$' }
    - has: { kind: jsx_expression, has: { kind: object } }
message: "Move this inline style into a stylesheet or a class"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const a = <p style={{ color: "red" }}>Hi</p>;
```

#### Do

```js
const a = <p className="warning">Hi</p>;
```

### JSX

#### Don't

```jsx
const a = <p style={{ color: "red" }}>Hi</p>;
```

#### Do

```jsx
const a = <p className="warning">Hi</p>;
```

### TSX

#### Don't

```tsx
const a = <p style={{ color: "red" }}>Hi</p>;
```

#### Do

```tsx
const a = <p className="warning">Hi</p>;
```
