---
id: react/no-non-scalable-viewport
title: "The viewport can be zoomed"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the author decides which zoom limits the page keeps"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noNonScalableViewport, "@html-eslint/no-non-scalable-viewport"]
since: 0.5.1
---

A `<meta name="viewport">` does not set `user-scalable=no` (or `0`) or a `maximum-scale` below 2. Behaviour of Biome's `noNonScalableViewport`.

**Why:** A page that cannot be zoomed locks out people with low vision; WCAG 1.4.4 asks for text that scales to 200%.

```codelaw
match:
  kind: jsx_self_closing_element
  all:
    - has: { field: name, kind: identifier, regex: '^meta$' }
    - has: { kind: jsx_attribute, all: [{ has: { kind: property_identifier, regex: '^name$' } }, { has: { kind: string, regex: 'viewport' } }] }
    - has:
        kind: jsx_attribute
        all:
          - has: { kind: property_identifier, regex: '^content$' }
          - has: { kind: string, regex: 'user-scalable\s*=\s*(no|0)|maximum-scale\s*=\s*[01](\.\d+)?\s*([,;"'']|$)' }
message: "This viewport stops people from zooming"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const m = <meta name="viewport" content="width=device-width, user-scalable=no" />;
```

#### Do

```js
const m = <meta name="viewport" content="width=device-width, initial-scale=1" />;
```

### JSX

#### Don't

```jsx
const m = <meta name="viewport" content="width=device-width, user-scalable=no" />;
```

#### Do

```jsx
const m = <meta name="viewport" content="width=device-width, initial-scale=1" />;
```

### TSX

#### Don't

```tsx
const m = <meta name="viewport" content="width=device-width, user-scalable=no" />;
```

#### Do

```tsx
const m = <meta name="viewport" content="width=device-width, initial-scale=1" />;
```
