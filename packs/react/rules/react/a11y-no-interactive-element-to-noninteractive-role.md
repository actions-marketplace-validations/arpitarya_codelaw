---
id: react/a11y-no-interactive-element-to-noninteractive-role
title: "Interactive elements keep an interactive role"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "removing the role keeps the interactive element and drops the semantics it claimed; the reviewer confirms the control is meant to stay, or swaps in a non-interactive element"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/no-interactive-element-to-noninteractive-role]
since: 0.5.0
---

A native interactive element (`<button>`, `<a>`, `<input>`, `<select>`, `<textarea>`, `<option>`) is not given a non-interactive role.

**Why:** The element still takes focus and clicks, but is announced as something that cannot be used.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^(a|button|input|select|textarea|option)$}
  - has: {kind: jsx_attribute, pattern: $R, regex: '^role=["''](article|banner|complementary|contentinfo|definition|dialog|document|feed|figure|form|group|heading|img|list|listitem|log|main|marquee|math|navigation|none|note|presentation|region|status|table|term|timer|toolbar|tooltip)["'']'}
message: Do not give an interactive element a non-interactive role
fix: [remove: $R]
```

## How to fix

1. Remove the role, or use a non-interactive element if the control is not meant to be used.

## Examples

### JavaScript

#### Don't

```js
const b = <button role="heading">Title</button>;
```

#### Fixed

```js
const b = <button >Title</button>;
```

#### Do

```js
const b = <h2>Title</h2>;
```

### JSX

#### Don't

```jsx
const b = <button role="heading">Title</button>;
```

#### Fixed

```jsx
const b = <button >Title</button>;
```

#### Do

```jsx
const b = <h2>Title</h2>;
```

### TSX

#### Don't

```tsx
const b = <button role="heading">Title</button>;
```

#### Fixed

```tsx
const b = <button >Title</button>;
```

#### Do

```tsx
const b = <h2>Title</h2>;
```
