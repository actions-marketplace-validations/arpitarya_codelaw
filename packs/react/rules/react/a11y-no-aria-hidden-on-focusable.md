---
id: react/a11y-no-aria-hidden-on-focusable
title: "No aria-hidden on a focusable element"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "either the element should be hidden from everyone or not hidden at all; the author decides"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-aria-hidden-on-focusable]
since: 0.5.0
---

An element that can take focus — a native control, a link with `href`, or one with a non-negative `tabIndex` — is not `aria-hidden="true"`.

**Why:** Keyboard users can still reach it, but screen readers announce nothing when they do.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {kind: jsx_attribute, regex: '^aria-hidden(=(["'']true|\{\s*true)|$)'}
  - any:
    - has: {field: name, regex: ^(button|input|select|textarea|summary)$}
      not:
        has: {kind: jsx_attribute, regex: '^(disabled|tabIndex=(["'']|\{\s*)-)'}
    - all:
      - has: {field: name, regex: ^a$}
      - has: {kind: jsx_attribute, regex: ^href\b}
      - not:
          has: {kind: jsx_attribute, regex: '^tabIndex=(["'']|\{\s*)-'}
    - has: {kind: jsx_attribute, regex: '^tabIndex=(["'']|\{\s*)[0-9]'}
message: Do not hide a focusable element with `aria-hidden`
```

## How to fix

1. Remove `aria-hidden`, or take the element out of the tab order (`tabIndex={-1}`, `disabled`) as well.

## Examples

### JavaScript

#### Don't

```js
const b = <button aria-hidden="true">Close</button>;
```

#### Do

```js
const b = <button>Close</button>;
```

### JSX

#### Don't

```jsx
const b = <button aria-hidden="true">Close</button>;
```

#### Do

```jsx
const b = <button>Close</button>;
```

### TSX

#### Don't

```tsx
const b = <button aria-hidden="true">Close</button>;
```

#### Do

```tsx
const b = <button>Close</button>;
```
