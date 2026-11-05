---
id: js/link-with-target-blank
title: "target=\"_blank\" with rel=\"noopener\""
languages: [jsx, tsx]
fix: suggest
fix_reason: "`noreferrer` also drops the Referer header, which an analytics setup may expect"
stability: preview
ported_from: [eslint-plugin-sonarjs/link-with-target-blank]
since: 0.5.0
---

A JSX link to an external URL with `target="_blank"` has `rel="noopener"`.

**Why:** Without it, the new page can navigate the opener with `window.opener.location`, a phishing vector.

```codelaw

match:
  kind: jsx_attribute
  regex: '^target=\{?["'']_blank["'']\}?$'
  inside: { any: [{kind: jsx_opening_element}, {kind: jsx_self_closing_element}], not: { has: { kind: jsx_attribute, regex: '^rel=.*noopener' } }, has: { kind: jsx_attribute, regex: '^href=\{?["''](https?:)?//' } }
message: "Add `rel=\"noopener noreferrer\"`; the opened page can otherwise reach `window.opener`"
fix:
  - insert-after: ' rel="noopener noreferrer"'
```

## Examples

### JSX

#### Don't

```jsx
<a href="https://example.com" target="_blank">site</a>;
```

#### Do

```jsx
<a href="https://example.com" target="_blank" rel="noopener noreferrer">site</a>;
```

### TSX

#### Don't

```tsx
<a href="https://example.com" target="_blank">site</a>;
```

#### Do

```tsx
<a href="https://example.com" target="_blank" rel="noopener noreferrer">site</a>;
```
