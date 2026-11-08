---
id: react/no-unescaped-entities
title: "No unescaped >, \", ' or } in JSX text"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/no-unescaped-entities]
since: 0.1.0
---

JSX text doesn't contain `>`, `"`, `'` or `}` unescaped.

**Why:** These characters are easy to misread as markup or code, and a stray `}` is usually a typo.

```codelaw
match:
  kind: jsx_text
  regex: '[>"''}]'
message: "Escape this character in JSX text, e.g. `&apos;` or `{\"'\"}`"
fix:
  - edit-text: { pattern: '>', replace: '&gt;' }
  - edit-text: { pattern: '"', replace: '&quot;' }
  - edit-text: { pattern: "'", replace: '&apos;' }
  - edit-text: { pattern: '\}', replace: '&#125;' }
```

## How to fix

1. Use an HTML entity (`&apos;`, `&quot;`, `&gt;`) or a string expression (`{"'"}`).

## Examples

### JavaScript

#### Don't

```js
const a = <p>Don't stop</p>;
```

#### Do

```js
const a = <p>Don&apos;t stop</p>;
```

### JSX

#### Don't

```jsx
const a = <p>Don't stop</p>;
```

#### Do

```jsx
const a = <p>Don&apos;t stop</p>;
```

### TSX

#### Don't

```tsx
const a = <p>Don't stop</p>;
```

#### Do

```tsx
const a = <p>Don&apos;t stop</p>;
```
