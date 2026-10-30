---
id: react/a11y-anchor-ambiguous-text
title: "Link text says where it goes"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the descriptive text is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/anchor-ambiguous-text]
since: 0.5.0
---

A link's whole text is not an ambiguous phrase such as "click here", "here", "link", "a link" or "learn more".

**Why:** Screen-reader users often list a page's links out of context; a list of "click here" says nothing.

```codelaw
match:
  kind: jsx_element
  all:
  - has:
      field: open_tag
      has: {field: name, regex: ^a$}
  - has: {kind: jsx_text, regex: '^\s*(?i:click here|here|link|a link|learn more)[.!?,:;]*\s*$'}
  - not:
      has:
        any:
        - {kind: jsx_element}
        - {kind: jsx_self_closing_element}
        - {kind: jsx_expression}
message: Ambiguous link text; say where the link goes
```

## How to fix

1. Replace the text with what the link opens ("Read the pricing guide").

## Examples

### JavaScript

#### Don't

```js
const a = <a href="/pricing">Click here</a>;
```

#### Do

```js
const a = <a href="/pricing">Read the pricing guide</a>;
```

### JSX

#### Don't

```jsx
const a = <a href="/pricing">Click here</a>;
```

#### Do

```jsx
const a = <a href="/pricing">Read the pricing guide</a>;
```

### TSX

#### Don't

```tsx
const a = <a href="/pricing">Click here</a>;
```

#### Do

```tsx
const a = <a href="/pricing">Read the pricing guide</a>;
```
