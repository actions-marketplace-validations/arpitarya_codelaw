---
id: react/no-suspicious-semicolon-in-jsx
title: "No stray semicolon after a JSX element"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the semicolon is rendered as text; whether it was meant is the author's call"
fix_pending: "suggest; needs computed-edit (W-162)"
stability: preview
ported_from: [biome/suspicious/noSuspiciousSemicolonInJsx]
since: 0.5.0
---

JSX text right after an element does not begin with a lone `;` on its line.

**Why:** A semicolon left after an element inside JSX is rendered on the page as text.

```codelaw
match:
  kind: jsx_text
  regex: '^;[ \t]*\r?\n'
  follows: { any: [{kind: jsx_element}, {kind: jsx_self_closing_element}] }
message: "This `;` is rendered as text; remove it"
```

## How to fix

1. Delete the semicolon.

## Examples

### JavaScript

#### Don't

```js
const page = (
  <div>
    <Header />;
    <Main />
  </div>
);
```

#### Do

```js
const page = (
  <div>
    <Header />
    <Main />
  </div>
);
```

### JSX

#### Don't

```jsx
const page = (
  <div>
    <Header />;
    <Main />
  </div>
);
```

#### Do

```jsx
const page = (
  <div>
    <Header />
    <Main />
  </div>
);
```

### TSX

#### Don't

```tsx
const page = (
  <div>
    <Header />;
    <Main />
  </div>
);
```

#### Do

```tsx
const page = (
  <div>
    <Header />
    <Main />
  </div>
);
```
