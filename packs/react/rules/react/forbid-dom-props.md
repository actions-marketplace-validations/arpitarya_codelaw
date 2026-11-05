---
id: react/forbid-dom-props
title: "No prop the project forbids on DOM elements"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "what replaces the prop is the project's convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  forbid: { type: "string[]", default: [], description: "prop names DOM elements may not take" }
ported_from: [eslint-plugin-react/forbid-dom-props]
since: 0.5.0
---

A DOM element doesn't take a prop named in `forbid`. With no option set, the rule does nothing.

**Why:** A project steers styling or ids through its own components.

```codelaw
native: react/forbid-dom-props
```

## How to fix

1. Remove the prop, or use the component the project provides.

## Examples

### JavaScript

#### Don't

```js options={"forbid":["id"]}
const e = <div id="x" />;
```

#### Do

```js options={"forbid":["id"]}
const e = <div className="x" />;
```

### JSX

#### Don't

```jsx options={"forbid":["id"]}
const e = <div id="x" />;
```

#### Do

```jsx options={"forbid":["id"]}
const e = <div className="x" />;
```

### TSX

#### Don't

```tsx options={"forbid":["id"]}
const e = <div id="x" />;
```

#### Do

```tsx options={"forbid":["id"]}
const e = <div className="x" />;
```
