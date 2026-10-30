---
id: react/a11y-no-distracting-elements
title: "No marquee or blink"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the replacement depends on what the element was for"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-distracting-elements]
since: 0.5.0
---

Neither `<marquee>` nor `<blink>` is used.

**Why:** Moving and flashing content is hard to read and can trigger seizures; both elements are obsolete.

```codelaw
match:
  any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
  has: { field: name, regex: '^(marquee|blink)$' }
message: "Do not use `<marquee>` or `<blink>`"
```

## How to fix

1. Show the content statically.

## Examples

### JavaScript

#### Don't

```js
const m = <marquee>Sale!</marquee>;
```

#### Do

```js
const m = <p>Sale!</p>;
```

### JSX

#### Don't

```jsx
const m = <marquee>Sale!</marquee>;
```

#### Do

```jsx
const m = <p>Sale!</p>;
```

### TSX

#### Don't

```tsx
const m = <marquee>Sale!</marquee>;
```

#### Do

```tsx
const m = <p>Sale!</p>;
```
