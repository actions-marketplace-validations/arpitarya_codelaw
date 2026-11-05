---
id: react/no-svg-without-title
title: "An SVG has a title or is hidden"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "adds aria-hidden=\"true\", which is right only when the SVG is decorative; the reviewer confirms that, or writes a title instead"
stability: preview
ported_from: [biome/a11y/noSvgWithoutTitle]
since: 0.5.0
---

An inline `<svg>` has a `<title>` inside it, an `aria-label` or `aria-labelledby`, or is hidden with `aria-hidden="true"` or `role="presentation"`.

**Why:** Screen readers announce an unlabelled SVG as "image" with no name, or read out its raw contents.

```codelaw
match:
  any:
    - kind: jsx_self_closing_element
      has: { field: name, regex: '^svg$', pattern: $N }
      not: { has: { any: [{kind: jsx_expression}, {kind: jsx_attribute, regex: '^(aria-label|aria-labelledby|aria-hidden|role=["''](presentation|none)|title)\b'}] } }
    - kind: jsx_element
      has:
        field: open_tag
        has: { field: name, regex: '^svg$', pattern: $N }
        not: { has: { any: [{kind: jsx_expression}, {kind: jsx_attribute, regex: '^(aria-label|aria-labelledby|aria-hidden|role=["''](presentation|none))\b'}] } }
      not:
        has:
          regex: '.'
          any:
            - { kind: jsx_self_closing_element, has: { field: name, regex: '^title$' } }
            - { kind: jsx_element, has: { field: open_tag, has: { field: name, regex: '^title$' } } }
fix:
  - insert-after: {target: $N, text: ' aria-hidden="true"'}
message: "Give this SVG a `<title>`, or hide it with `aria-hidden=\"true\"` if it is decorative"
```

## How to fix

1. Add `<title>` with a short name as the SVG's first child, or `aria-hidden="true"` if it is decorative.

## Examples

### JavaScript

#### Don't

```js
const icon = <svg viewBox="0 0 8 8"><path d="M0 0h8v8H0z" /></svg>;
```

#### Fixed

```js
const icon = <svg aria-hidden="true" viewBox="0 0 8 8"><path d="M0 0h8v8H0z" /></svg>;
```

#### Do

```js
const icon = <svg viewBox="0 0 8 8"><title>Stop</title><path d="M0 0h8v8H0z" /></svg>;
```

### JSX

#### Don't

```jsx
const icon = <svg viewBox="0 0 8 8"><path d="M0 0h8v8H0z" /></svg>;
```

#### Fixed

```jsx
const icon = <svg aria-hidden="true" viewBox="0 0 8 8"><path d="M0 0h8v8H0z" /></svg>;
```

#### Do

```jsx
const icon = <svg viewBox="0 0 8 8"><title>Stop</title><path d="M0 0h8v8H0z" /></svg>;
```

### TSX

#### Don't

```tsx
const icon = <svg viewBox="0 0 8 8"><path d="M0 0h8v8H0z" /></svg>;
```

#### Fixed

```tsx
const icon = <svg aria-hidden="true" viewBox="0 0 8 8"><path d="M0 0h8v8H0z" /></svg>;
```

#### Do

```tsx
const icon = <svg viewBox="0 0 8 8"><title>Stop</title><path d="M0 0h8v8H0z" /></svg>;
```
