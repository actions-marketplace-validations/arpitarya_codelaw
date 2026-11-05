---
id: react/a11y-no-aria-hidden-on-focusable
title: "No aria-hidden on a focusable element"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing aria-hidden exposes the element to assistive technology while tabIndex -1 keeps it hidden but unfocusable; the reviewer picks"
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
  - has: {kind: jsx_attribute, regex: '^aria-hidden(=(["'']true|\{\s*true)|$)', pattern: $H}
  - any:
    - has: {field: name, regex: ^(button|input|select|textarea|summary)$, pattern: $N}
      not:
        has: {kind: jsx_attribute, regex: '^(disabled|tabIndex=(["'']|\{\s*)-)'}
    - all:
      - has: {field: name, regex: ^a$, pattern: $N}
      - has: {kind: jsx_attribute, regex: ^href\b}
      - not:
          has: {kind: jsx_attribute, regex: '^tabIndex=(["'']|\{\s*)-'}
    - has: {kind: jsx_attribute, regex: '^tabIndex=(["'']|\{\s*)[0-9]', pattern: $T}
message: Do not hide a focusable element with `aria-hidden`
fix:
  choices:
    - label: remove aria-hidden
      fix:
        - remove: $H
    - label: take out of tab order
      fix:
        - insert-after: { target: $N, text: " tabIndex={-1}" }
        - replace: { target: $T, with: "tabIndex={-1}" }
```

## How to fix

1. Remove `aria-hidden`, or take the element out of the tab order (`tabIndex={-1}`, `disabled`) as well.

## Examples

### JavaScript

#### Don't

```js
const b = <button aria-hidden="true">Close</button>;
```

#### Fixed (remove aria-hidden)

```js
const b = <button >Close</button>;
```

#### Fixed (take out of tab order)

```js
const b = <button tabIndex={-1} aria-hidden="true">Close</button>;
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

#### Fixed (remove aria-hidden)

```jsx
const b = <button >Close</button>;
```

#### Fixed (take out of tab order)

```jsx
const b = <button tabIndex={-1} aria-hidden="true">Close</button>;
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

#### Fixed (remove aria-hidden)

```tsx
const b = <button >Close</button>;
```

#### Fixed (take out of tab order)

```tsx
const b = <button tabIndex={-1} aria-hidden="true">Close</button>;
```

#### Do

```tsx
const b = <button>Close</button>;
```
