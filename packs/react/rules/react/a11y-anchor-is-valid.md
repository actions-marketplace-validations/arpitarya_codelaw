---
id: react/a11y-anchor-is-valid
title: "A link goes somewhere"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a button takes the click without navigating, so the reviewer confirms the link runs an action rather than needing a real URL"
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
  - has: {field: name, regex: ^a$, pattern: $N}
  - any:
    - {kind: jsx_self_closing_element}
    - inside:
        kind: jsx_element
        has: {field: close_tag, has: {field: name, regex: ^a$, pattern: $CN}}
  - not:
      has: {kind: jsx_expression}
  - any:
    - has: {kind: jsx_attribute, regex: '^href=(\{\s*)?["''`](#|javascript:)', pattern: $H}
      not:
        has: {kind: jsx_attribute, regex: '^href=(\{\s*)?["''`]#[^"''`]'}
    - has: {kind: jsx_attribute, regex: '^href=(\{\s*)?["''`]javascript:', pattern: $H}
    - not:
        has: {kind: jsx_attribute, regex: ^href\b}
      has: {kind: jsx_attribute, regex: ^onClick\b}
message: This link goes nowhere; give it a real `href`, or use a `<button>`
fix:
  - replace: { target: $N, with: 'button type="button"' }
  - remove: $H
  - replace: { target: $CN, with: "button" }
```

## How to fix

1. Give the link a URL it navigates to, or replace it with a `<button>` if it runs an action.

## Examples

### JavaScript

#### Don't

```js
const a = <a href="#" onClick={open}>Open</a>;
```

#### Fixed

```js
const a = <button type="button"  onClick={open}>Open</button>;
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

#### Fixed

```jsx
const a = <button type="button"  onClick={open}>Open</button>;
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

#### Fixed

```tsx
const a = <button type="button"  onClick={open}>Open</button>;
```

#### Do

```tsx
const a = <button type="button" onClick={open}>Open</button>;
```
