---
id: react/a11y-click-events-have-key-events
title: "A clickable element also answers the keyboard"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "a button changes the element's markup and styling, and the copied key handler assumes Enter and Space should click it; the reviewer picks"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/click-events-have-key-events]
since: 0.5.0
---

A non-interactive DOM element with an `onClick` also has an `onKeyDown`, `onKeyUp` or `onKeyPress`.

**Why:** Keyboard users cannot click; without a key handler the action is out of their reach.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: '^[a-z]', pattern: $N}
  - any:
    - {kind: jsx_self_closing_element}
    - inside:
        kind: jsx_element
        has: {field: close_tag, has: {field: name, pattern: $CN}}
  - not:
      has: {field: name, regex: ^(a|area|button|input|select|textarea|option|summary|details|iframe|embed|audio|video)$}
  - has: {kind: jsx_attribute, regex: ^onClick\b, pattern: $C, has: {kind: jsx_expression, has: {pattern: $HE}}}
  - not:
      has: {kind: jsx_attribute, regex: '^(onKeyDown|onKeyUp|onKeyPress)\b|^aria-hidden(=(["''])true|=\{\s*true|$)|^role=["''](presentation|none)'}
  - not:
      has: {kind: jsx_expression}
message: Add a keyboard handler beside `onClick`, or use a `<button>`
fix:
  choices:
    - label: use a button
      fix:
        - replace: { target: $N, with: 'button type="button"' }
        - replace: { target: $CN, with: "button" }
    - label: add key handler
      fix:
        - insert-after: { target: $C, text: ' tabIndex={0} onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") ($HE)(e); }}' }
```

## How to fix

1. Use a `<button>` (it handles Enter and Space itself), or add `onKeyDown` and a `tabIndex`.

## Examples

### JavaScript

#### Don't

```js
const c = <div onClick={open}>Open</div>;
```

#### Fixed (use a button)

```js
const c = <button type="button" onClick={open}>Open</button>;
```

#### Fixed (add key handler)

```js
const c = <div onClick={open} tabIndex={0} onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") (open)(e); }}>Open</div>;
```

#### Do

```js
const c = <button type="button" onClick={open}>Open</button>;
```

### JSX

#### Don't

```jsx
const c = <div onClick={open}>Open</div>;
```

#### Fixed (use a button)

```jsx
const c = <button type="button" onClick={open}>Open</button>;
```

#### Fixed (add key handler)

```jsx
const c = <div onClick={open} tabIndex={0} onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") (open)(e); }}>Open</div>;
```

#### Do

```jsx
const c = <button type="button" onClick={open}>Open</button>;
```

### TSX

#### Don't

```tsx
const c = <div onClick={open}>Open</div>;
```

#### Fixed (use a button)

```tsx
const c = <button type="button" onClick={open}>Open</button>;
```

#### Fixed (add key handler)

```tsx
const c = <div onClick={open} tabIndex={0} onKeyDown={(e) => { if (e.key === "Enter" || e.key === " ") (open)(e); }}>Open</div>;
```

#### Do

```tsx
const c = <button type="button" onClick={open}>Open</button>;
```
