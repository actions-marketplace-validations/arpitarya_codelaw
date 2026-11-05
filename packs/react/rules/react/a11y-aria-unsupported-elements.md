---
id: react/a11y-aria-unsupported-elements
title: "No ARIA on elements that cannot carry it"
languages: [javascript, jsx, tsx]
fix: auto
fix_reason: "the element is never rendered or has no role, so removing the attribute changes nothing for assistive technology"
stability: preview
ported_from: [eslint-plugin-jsx-a11y/aria-unsupported-elements]
since: 0.5.0
---

Elements that are never rendered or have no accessible role — `<meta>`, `<html>`, `<script>`, `<style>` and the like — carry no `role` or `aria-*` attribute.

**Why:** Assistive technology ignores ARIA on these elements, so the attribute does nothing but suggest it does.

```codelaw
match:
  kind: jsx_attribute
  regex: '^(role|aria-[a-z]+)\b'
  inside:
    any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}]
    has: { field: name, regex: '^(base|col|colgroup|head|html|link|meta|noembed|noscript|param|picture|script|source|style|template|title|track)$' }
message: "This element does not support ARIA; remove `$$MATCH`"
fix: [remove]
```

## How to fix

1. Remove the attribute, or move it to the element it was meant for.

## Examples

### JavaScript

#### Don't

```js
const m = <meta charset="utf-8" aria-hidden="true" />;
```

#### Fixed

```js
const m = <meta charset="utf-8"  />;
```

#### Do

```js
const m = <meta charset="utf-8" />;
```

### JSX

#### Don't

```jsx
const m = <meta charset="utf-8" aria-hidden="true" />;
```

#### Fixed

```jsx
const m = <meta charset="utf-8"  />;
```

#### Do

```jsx
const m = <meta charset="utf-8" />;
```

### TSX

#### Don't

```tsx
const m = <meta charset="utf-8" aria-hidden="true" />;
```

#### Fixed

```tsx
const m = <meta charset="utf-8"  />;
```

#### Do

```tsx
const m = <meta charset="utf-8" />;
```
