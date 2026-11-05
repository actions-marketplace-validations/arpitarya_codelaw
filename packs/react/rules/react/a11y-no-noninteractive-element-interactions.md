---
id: react/a11y-no-noninteractive-element-interactions
title: "Non-interactive elements have no handlers"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the fix is a native control or an interactive role with focus, and that is the author's choice"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-noninteractive-element-interactions]
since: 0.5.0
---

A non-interactive element (`<li>`, `<h1>`, `<img>`, `<main>`, …) without a role has no click or key handler.

**Why:** Its handler cannot be reached by keyboard, and assistive technology does not announce it as usable.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^(main|article|aside|nav|header|footer|section|h[1-6]|p|blockquote|figure|figcaption|img|li|ul|ol|dl|dt|dd|pre|table|tbody|thead|tfoot|tr|td|th|form|fieldset|label|hr|address|caption|dialog|legend|meter|output|progress|time)$'}
  - has: {kind: jsx_attribute, regex: ^(onClick|onMouseDown|onMouseUp|onKeyPress|onKeyDown|onKeyUp)\b}
  - not:
      has: {kind: jsx_attribute, regex: ^role\b}
  - not:
      has: {kind: jsx_expression}
message: Do not put a handler on a non-interactive element; use a `<button>` inside it
```

## How to fix

1. Put a `<button>` (or link) inside the element and move the handler to it.

## Examples

### JavaScript

#### Don't

```js
const i = <li onClick={pick}>One</li>;
```

#### Do

```js
const i = <li><button type="button" onClick={pick}>One</button></li>;
```

### JSX

#### Don't

```jsx
const i = <li onClick={pick}>One</li>;
```

#### Do

```jsx
const i = <li><button type="button" onClick={pick}>One</button></li>;
```

### TSX

#### Don't

```tsx
const i = <li onClick={pick}>One</li>;
```

#### Do

```tsx
const i = <li><button type="button" onClick={pick}>One</button></li>;
```
