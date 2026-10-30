---
id: next/no-sync-scripts
title: "No synchronous scripts"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "`async`, `defer` or `next/script` each change when the script runs"
stability: preview
ported_from: ["@next/eslint-plugin-next/no-sync-scripts"]
since: 0.5.0
---

A `<script>` with a `src` is `async` or `defer` (or a module).

**Why:** A synchronous script blocks the page from rendering until it has downloaded and run.

```codelaw
match:
  all:
  - any:
    - {kind: jsx_opening_element}
    - {kind: jsx_self_closing_element}
  - has: {field: name, regex: ^script$}
  - has: {kind: jsx_attribute, regex: ^src\b}
  - not:
      has:
        any:
        - {kind: jsx_expression}
        - {kind: jsx_attribute, regex: '^(async|defer)\b|^type=["'']module["'']'}
message: Make this script `async` or `defer`, or use `next/script`
```

## How to fix

1. Add `async` or `defer`, or use `<Script>` from `next/script`.

## Examples

### JavaScript

#### Don't

```js
const s = <script src="/a.js" />;
```

#### Do

```js
const s = <script src="/a.js" defer />;
```

### JSX

#### Don't

```jsx
const s = <script src="/a.js" />;
```

#### Do

```jsx
const s = <script src="/a.js" defer />;
```

### TSX

#### Don't

```tsx
const s = <script src="/a.js" />;
```

#### Do

```tsx
const s = <script src="/a.js" defer />;
```
