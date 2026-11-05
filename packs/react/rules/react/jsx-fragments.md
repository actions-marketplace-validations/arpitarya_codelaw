---
id: react/jsx-fragments
title: "<>…</> rather than <React.Fragment>"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/jsx-fragments]
since: 0.5.0
---

A fragment with no `key` is written with the short syntax `<>…</>`, not `<React.Fragment>…</React.Fragment>` or `<Fragment>…</Fragment>`.

**Why:** The short syntax is the same fragment with less to read.

```codelaw
match:
  kind: jsx_element
  all:
    - has: { field: open_tag, regex: '^<(React\.)?Fragment\s*>$', pattern: $O }
    - has: { field: close_tag, pattern: $C }
message: "Write this fragment as `<>…</>`"
fix:
  - replace: { target: $O, with: "<>" }
  - replace: { target: $C, with: "</>" }
```

## Examples

### JavaScript

#### Don't

```js
const l = <React.Fragment><A /><B /></React.Fragment>;
```

#### Do

```js
const l = <><A /><B /></>;
```

### JSX

#### Don't

```jsx
const l = <React.Fragment><A /><B /></React.Fragment>;
```

#### Do

```jsx
const l = <><A /><B /></>;
```

### TSX

#### Don't

```tsx
const l = <React.Fragment><A /><B /></React.Fragment>;
```

#### Do

```tsx
const l = <><A /><B /></>;
```
