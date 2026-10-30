---
id: react/a11y-iframe-has-title
title: "An iframe has a title"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the frame's title is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/iframe-has-title]
since: 0.5.0
---

An `<iframe>` carries a non-empty `title`.

**Why:** The title is how a screen-reader user learns what the frame holds before entering it.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^iframe$}
  - not:
      has: {kind: jsx_expression}
  - any:
    - not:
        has: {kind: jsx_attribute, regex: ^title\b}
    - has: {kind: jsx_attribute, regex: '^title=(["''])\s*["'']$'}
message: Give this `<iframe>` a `title` that says what it shows
```

## How to fix

1. Add a `title` describing the frame's content.

## Examples

### JavaScript

#### Don't

```js
const v = <iframe src="/video" />;
```

#### Do

```js
const v = <iframe src="/video" title="Product demo" />;
```

### JSX

#### Don't

```jsx
const v = <iframe src="/video" />;
```

#### Do

```jsx
const v = <iframe src="/video" title="Product demo" />;
```

### TSX

#### Don't

```tsx
const v = <iframe src="/video" />;
```

#### Do

```tsx
const v = <iframe src="/video" title="Product demo" />;
```
