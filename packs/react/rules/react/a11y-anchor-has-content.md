---
id: react/a11y-anchor-has-content
title: "A link has content"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the link's text is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/anchor-has-content]
since: 0.5.0
---

An `<a>` has text, an element or an `aria-label` inside it.

**Why:** A link with nothing in it is announced as "link" and nothing more.

```codelaw
match:
  any:
  - kind: jsx_self_closing_element
    all:
    - has: {field: name, regex: ^a$}
    - not:
        has: {kind: jsx_expression}
    - not:
        has: {kind: jsx_attribute, regex: ^(aria-label|aria-labelledby|title|dangerouslySetInnerHTML|children)\b}
  - kind: jsx_element
    has:
      field: open_tag
      has: {field: name, regex: ^a$}
      not:
        has: {kind: jsx_attribute, regex: ^(aria-label|aria-labelledby|title|dangerouslySetInnerHTML)\b}
    not:
      has:
        any:
        - {kind: jsx_element}
        - {kind: jsx_self_closing_element}
        - {kind: jsx_expression}
        - {kind: jsx_text, regex: \S}
message: This link has no content; put text inside it or give it an `aria-label`
```

## How to fix

1. Put the link's text between the tags, or add an `aria-label`.

## Examples

### JavaScript

#### Don't

```js
const a = <a href="/home"></a>;
```

#### Do

```js
const a = <a href="/home">Home</a>;
```

### JSX

#### Don't

```jsx
const a = <a href="/home"></a>;
```

#### Do

```jsx
const a = <a href="/home">Home</a>;
```

### TSX

#### Don't

```tsx
const a = <a href="/home"></a>;
```

#### Do

```tsx
const a = <a href="/home">Home</a>;
```
