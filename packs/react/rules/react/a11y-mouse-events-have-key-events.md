---
id: react/a11y-mouse-events-have-key-events
title: "Hover handlers have focus twins"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the focus handler's behaviour is the author's"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/mouse-events-have-key-events]
since: 0.5.0
---

A DOM element with `onMouseOver` also has `onFocus`, and one with `onMouseOut` also has `onBlur`.

**Why:** Whatever appears on hover must also appear on keyboard focus, or keyboard users never see it.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]'}
  - not:
      has: {kind: jsx_expression}
  - any:
    - has: {kind: jsx_attribute, regex: ^onMouseOver\b}
      not:
        has: {kind: jsx_attribute, regex: ^onFocus\b}
    - has: {kind: jsx_attribute, regex: ^onMouseOut\b}
      not:
        has: {kind: jsx_attribute, regex: ^onBlur\b}
message: Pair `onMouseOver` with `onFocus` and `onMouseOut` with `onBlur`
```

## How to fix

1. Add `onFocus` (and `onBlur`) running the same code as the mouse handler.

## Examples

### JavaScript

#### Don't

```js
const t = <span onMouseOver={show}>?</span>;
```

#### Do

```js
const t = <span onMouseOver={show} onFocus={show}>?</span>;
```

### JSX

#### Don't

```jsx
const t = <span onMouseOver={show}>?</span>;
```

#### Do

```jsx
const t = <span onMouseOver={show} onFocus={show}>?</span>;
```

### TSX

#### Don't

```tsx
const t = <span onMouseOver={show}>?</span>;
```

#### Do

```tsx
const t = <span onMouseOver={show} onFocus={show}>?</span>;
```
