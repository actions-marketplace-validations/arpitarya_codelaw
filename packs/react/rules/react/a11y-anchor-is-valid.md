---
id: react/a11y-anchor-is-valid
title: "A link goes somewhere"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the fix is a real URL or a `<button>`, and which one is the author's choice"
fix_pending: "suggest"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/anchor-is-valid]
since: 0.5.0
---

An `<a>` has an `href` that navigates: not `#`, not `javascript:`, and not missing on a link that only has an `onClick`.

**Why:** A link that does not navigate is a button pretending to be a link: it cannot be opened in a new tab and announces the wrong role.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^a$}
  - not:
      has: {kind: jsx_expression}
  - any:
    - has: {kind: jsx_attribute, regex: '^href=(\{\s*)?["''`](#|javascript:)'}
      not:
        has: {kind: jsx_attribute, regex: '^href=(\{\s*)?["''`]#[^"''`]'}
    - has: {kind: jsx_attribute, regex: '^href=(\{\s*)?["''`]javascript:'}
    - not:
        has: {kind: jsx_attribute, regex: ^href\b}
      has: {kind: jsx_attribute, regex: ^onClick\b}
message: This link goes nowhere; give it a real `href`, or use a `<button>`
```

## How to fix

1. Give the link a URL it navigates to, or replace it with a `<button>` if it runs an action.

## Examples

### JavaScript

#### Don't

```js
const a = <a href="#" onClick={open}>Open</a>;
```

#### Do

```js
const a = <button type="button" onClick={open}>Open</button>;
```

### JSX

#### Don't

```jsx
const a = <a href="#" onClick={open}>Open</a>;
```

#### Do

```jsx
const a = <button type="button" onClick={open}>Open</button>;
```

### TSX

#### Don't

```tsx
const a = <a href="#" onClick={open}>Open</a>;
```

#### Do

```tsx
const a = <button type="button" onClick={open}>Open</button>;
```
