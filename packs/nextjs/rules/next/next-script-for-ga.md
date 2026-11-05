---
id: next/next-script-for-ga
title: "Google Analytics through next/third-parties"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "moving to the component changes how the tag is configured"
fix_pending: "suggest; needs add-import (W-162)"
stability: preview
ported_from: ["@next/eslint-plugin-next/next-script-for-ga"]
since: 0.5.0
---

Google Analytics is not loaded with a raw `<script>` tag.

**Why:** A raw tag blocks or races page loading; `@next/third-parties/google` (or `next/script`) loads it at the right time.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^script$}
  - has: {kind: jsx_attribute, regex: (googletagmanager\.com/gtag/js|google-analytics\.com/analytics\.js)}
message: Load Google Analytics with `@next/third-parties/google` or `next/script`
```

## How to fix

1. Use `<GoogleAnalytics gaId="…" />` from `@next/third-parties/google`.

## Examples

### JavaScript

#### Don't

```js
const ga = <script async src="https://www.googletagmanager.com/gtag/js?id=G-1" />;
```

#### Do

```js
const ga = <GoogleAnalytics gaId="G-1" />;
```

### JSX

#### Don't

```jsx
const ga = <script async src="https://www.googletagmanager.com/gtag/js?id=G-1" />;
```

#### Do

```jsx
const ga = <GoogleAnalytics gaId="G-1" />;
```

### TSX

#### Don't

```tsx
const ga = <script async src="https://www.googletagmanager.com/gtag/js?id=G-1" />;
```

#### Do

```tsx
const ga = <GoogleAnalytics gaId="G-1" />;
```
