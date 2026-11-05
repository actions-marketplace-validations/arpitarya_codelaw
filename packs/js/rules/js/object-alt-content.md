---
id: js/object-alt-content
title: "object elements have fallback content"
languages: [jsx, tsx]
fix: manual
fix_reason: "the text is human-written"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/object-alt-content]
since: 0.5.0
---

A JSX `<object>` element has content (fallback text).

**Why:** Screen readers announce the fallback content; an empty object is skipped or announced as unlabelled.

```codelaw

match:
  kind: jsx_self_closing_element
  has: { field: name, regex: '^object$' }
message: "Give the `<object>` fallback text for assistive technology"
```

## How to fix

1. Write a short description of the embedded content between the tags.

## Examples

### JSX

#### Don't

```jsx
const v = <object data="chart.svg" />;
```

#### Do

```jsx
const v = <object data="chart.svg">Sales by month</object>;
```

### TSX

#### Don't

```tsx
const v = <object data="chart.svg" />;
```

#### Do

```tsx
const v = <object data="chart.svg">Sales by month</object>;
```
