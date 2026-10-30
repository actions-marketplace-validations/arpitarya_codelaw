---
id: react/no-adjacent-inline-elements
title: "Inline elements are separated by a space"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "the space it inserts is rendered, which is the point but changes the output"
stability: preview
ported_from: [eslint-plugin-react/no-adjacent-inline-elements]
since: 0.5.0
---

Two inline elements (`<a>`, `<span>`, `<strong>`, …) do not sit directly next to each other with nothing between them.

**Why:** With no whitespace between them they render glued together (`<b>a</b><i>b</i>` shows `ab`), which is rarely meant.

```codelaw
utils:
  inline:
    any:
      - { kind: jsx_element, has: { field: open_tag, has: { field: name, regex: '^(a|abbr|acronym|b|bdo|big|button|cite|code|dfn|em|i|img|input|kbd|label|map|object|q|samp|script|select|small|span|strong|sub|sup|textarea|tt|var)$' } } }
      - { kind: jsx_self_closing_element, has: { field: name, regex: '^(a|abbr|acronym|b|bdo|big|button|cite|code|dfn|em|i|img|input|kbd|label|map|object|q|samp|script|select|small|span|strong|sub|sup|textarea|tt|var)$' } }
match:
  matches: inline
  precedes: { matches: inline }
message: "These inline elements render with no space between them"
fix:
  - insert-after: '{" "}'
```

## Examples

### JavaScript

#### Don't

```js
const t = <p><strong>Name</strong><em>Ada</em></p>;
```

#### Do

```js
const t = <p><strong>Name</strong> <em>Ada</em></p>;
```

#### Fixed

```js
const t = <p><strong>Name</strong>{" "}<em>Ada</em></p>;
```

### JSX

#### Don't

```jsx
const t = <p><strong>Name</strong><em>Ada</em></p>;
```

#### Do

```jsx
const t = <p><strong>Name</strong> <em>Ada</em></p>;
```

#### Fixed

```jsx
const t = <p><strong>Name</strong>{" "}<em>Ada</em></p>;
```

### TSX

#### Don't

```tsx
const t = <p><strong>Name</strong><em>Ada</em></p>;
```

#### Do

```tsx
const t = <p><strong>Name</strong> <em>Ada</em></p>;
```

#### Fixed

```tsx
const t = <p><strong>Name</strong>{" "}<em>Ada</em></p>;
```
