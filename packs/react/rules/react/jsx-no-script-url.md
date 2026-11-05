---
id: react/jsx-no-script-url
title: "No javascript: URLs in JSX"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "security: the code the URL runs has to become a real handler"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-react/jsx-no-script-url]
since: 0.5.0
---

No JSX attribute is set to a `javascript:` URL.

**Why:** React 19 blocks these URLs, and before that they were an injection path; a click handler does the same job safely.

```codelaw
match:
  kind: jsx_attribute
  has: { kind: string, regex: '^["'']\s*javascript:' }
message: "No `javascript:` URLs; use an event handler"
```

## How to fix

1. Move the code into an `onClick` handler.
2. Give the link a real `href`, or make it a `<button type="button">`.

## Examples

### JavaScript

#### Don't

```js
const a = <a href="javascript:void(0)">Open</a>;
```

#### Do

```js
const a = <button type="button" onClick={open}>Open</button>;
```

### JSX

#### Don't

```jsx
const a = <a href="javascript:void(0)">Open</a>;
```

#### Do

```jsx
const a = <button type="button" onClick={open}>Open</button>;
```

### TSX

#### Don't

```tsx
const a = <a href="javascript:void(0)">Open</a>;
```

#### Do

```tsx
const a = <button type="button" onClick={open}>Open</button>;
```
