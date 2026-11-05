---
id: react/a11y-alt-text
title: "Images and image-like elements carry alternative text"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "an `<img>` hidden with aria-hidden is decorative, so an empty `alt` changes nothing for assistive technology; any other image needs an `alt` that says what it shows, so the reviewer confirms it is decorative"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/alt-text]
since: 0.5.0
---

`<img>`, `<area>` and `<input type="image">` carry an `alt` (an empty one for a decorative image) or an `aria-label`, and `<object>` a `title` or `aria-label`.

**Why:** A screen reader announces an image without alternative text by its file name, or not at all.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - not:
      has: {kind: jsx_expression}
  - not:
      has: {kind: jsx_attribute, regex: ^(aria-label|aria-labelledby)\b}
  - any:
    - has: {field: name, regex: ^img$, pattern: $N}
      not:
        has: {kind: jsx_attribute, regex: '^(alt\b|role=["'']?(presentation|none))'}
    - has: {field: name, regex: ^area$}
      not:
        has: {kind: jsx_attribute, regex: '^(alt\b|role=["'']?(presentation|none))'}
    - all:
      - has: {field: name, regex: ^input$}
      - has: {kind: jsx_attribute, regex: '^type=["'']image["'']'}
      - not:
          has: {kind: jsx_attribute, regex: ^alt\b}
    - has: {field: name, regex: ^object$}
      not:
        has: {kind: jsx_attribute, regex: ^title\b}
auto_when:
  all:
  - has: {field: name, regex: ^img$}
  - has: {kind: jsx_attribute, regex: '^aria-hidden(=(["'']true|\{\s*true\s*\})|$)'}
message: 'Give this element alternative text: an `alt` (empty for a decorative image) or an `aria-label`'
fix:
  - insert-after: { target: $N, text: ' alt=""' }
```

## How to fix

1. Add an `alt` that says what the image shows, or `alt=""` if it is decorative.

## Examples

### JavaScript

#### Don't

```js
const logo = <img src="logo.png" />;
```

#### Fixed

```js
const logo = <img alt="" src="logo.png" />;
```

#### Do

```js
const logo = <img src="logo.png" alt="Acme" />;
```

### JSX

#### Don't

```jsx
const logo = <img src="logo.png" />;
```

#### Fixed

```jsx
const logo = <img alt="" src="logo.png" />;
```

#### Do

```jsx
const logo = <img src="logo.png" alt="Acme" />;
```

### TSX

#### Don't

```tsx
const logo = <img src="logo.png" />;
```

#### Fixed

```tsx
const logo = <img alt="" src="logo.png" />;
```

#### Do

```tsx
const logo = <img src="logo.png" alt="Acme" />;
```
