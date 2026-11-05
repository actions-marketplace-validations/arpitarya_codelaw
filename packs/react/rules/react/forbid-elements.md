---
id: react/forbid-elements
title: "No element the project forbids"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "which element replaces it is the project's convention, which the code doesn't hold"
fix_exception: value
stability: preview
options:
  forbid: { type: "string[]", default: [], description: "element or component names that may not be used" }
ported_from: [eslint-plugin-react/forbid-elements]
since: 0.5.0
---

An element named in `forbid` isn't used. With no option set, the rule does nothing.

**Why:** A project replaces raw elements with its own components.

```codelaw
native: react/forbid-elements
```

## How to fix

1. Use the component the project provides.

## Examples

### JavaScript

#### Don't

```js options={"forbid":["button"]}
const e = <button>Go</button>;
```

#### Do

```js options={"forbid":["button"]}
const e = <Button>Go</Button>;
```

### JSX

#### Don't

```jsx options={"forbid":["button"]}
const e = <button>Go</button>;
```

#### Do

```jsx options={"forbid":["button"]}
const e = <Button>Go</Button>;
```

### TSX

#### Don't

```tsx options={"forbid":["button"]}
const e = <button>Go</button>;
```

#### Do

```tsx options={"forbid":["button"]}
const e = <Button>Go</Button>;
```
