---
id: react/a11y-no-distracting-elements
title: "No marquee or blink"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "`<blink>` is unsupported by current browsers and renders as an inline span, so `span` changes nothing; `<marquee>` still scrolls, so its `div` replacement is only suggested"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-distracting-elements]
since: 0.5.0
---

Neither `<marquee>` nor `<blink>` is used.

**Why:** Moving and flashing content is hard to read and can trigger seizures; both elements are obsolete.

```codelaw
match:
  any:
  - all:
    - any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
    - has: { field: name, regex: '^blink$', pattern: $B }
    - any:
      - {kind: jsx_self_closing_element}
      - inside:
          kind: jsx_element
          has: {field: close_tag, has: {field: name, regex: '^blink$', pattern: $BC}}
  - all:
    - any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
    - has: { field: name, regex: '^marquee$', pattern: $M }
    - any:
      - {kind: jsx_self_closing_element}
      - inside:
          kind: jsx_element
          has: {field: close_tag, has: {field: name, regex: '^marquee$', pattern: $MC}}
auto_when:
  has: { field: name, regex: '^blink$' }
message: "Do not use `<marquee>` or `<blink>`"
fix:
  - replace: { target: $B, with: "span" }
  - replace: { target: $BC, with: "span" }
  - replace: { target: $M, with: "div" }
  - replace: { target: $MC, with: "div" }
```

## How to fix

1. Show the content statically.

## Examples

### JavaScript

#### Don't

```js
const m = <marquee>Sale!</marquee>;
```

#### Fixed

```js
const m = <div>Sale!</div>;
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

#### Fixed

```jsx
const m = <div>Sale!</div>;
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

#### Fixed

```tsx
const m = <div>Sale!</div>;
```

#### Do

```tsx
const m = <p>Sale!</p>;
```
