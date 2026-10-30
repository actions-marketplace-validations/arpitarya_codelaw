---
id: react/a11y-heading-has-content
title: "A heading has content"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the heading's text is the author's"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/heading-has-content]
since: 0.5.0
---

`<h1>` to `<h6>` have text, an element or an `aria-label` inside them.

**Why:** An empty heading is announced as a heading with no name and breaks navigation by headings.

```codelaw
match:
  any:
  - kind: jsx_self_closing_element
    all:
    - has: {field: name, regex: '^h[1-6]$'}
    - not:
        has: {kind: jsx_expression}
    - not:
        has: {kind: jsx_attribute, regex: ^(aria-label|aria-labelledby|title|dangerouslySetInnerHTML|children)\b}
  - kind: jsx_element
    has:
      field: open_tag
      has: {field: name, regex: '^h[1-6]$'}
      not:
        has: {kind: jsx_attribute, regex: ^(aria-label|aria-labelledby|dangerouslySetInnerHTML)\b}
    not:
      has:
        any:
        - {kind: jsx_element}
        - {kind: jsx_self_closing_element}
        - {kind: jsx_expression}
        - {kind: jsx_text, regex: \S}
message: This heading is empty; put its text inside it
```

## How to fix

1. Put the heading's text between the tags, or remove the heading.

## Examples

### JavaScript

#### Don't

```js
const h = <h2 />;
```

#### Do

```js
const h = <h2>Settings</h2>;
```

### JSX

#### Don't

```jsx
const h = <h2 />;
```

#### Do

```jsx
const h = <h2>Settings</h2>;
```

### TSX

#### Don't

```tsx
const h = <h2 />;
```

#### Do

```tsx
const h = <h2>Settings</h2>;
```
