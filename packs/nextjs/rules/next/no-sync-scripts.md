---
id: next/no-sync-scripts
title: "No synchronous scripts"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "`defer` and `async` change when the script runs; the reviewer picks one (moving to `next/script` stays by hand)"
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
  - has: {kind: jsx_attribute, regex: ^src\b, pattern: $SRC}
  - not:
      has:
        any:
        - {kind: jsx_expression}
        - {kind: jsx_attribute, regex: '^(async|defer)\b|^type=["'']module["'']'}
message: Make this script `async` or `defer`, or use `next/script`
fix:
  choices:
    - label: defer
      default: true
      fix:
        - insert-after: { target: $SRC, text: " defer" }
    - label: async
      fix:
        - insert-after: { target: $SRC, text: " async" }
```

## How to fix

1. Add `async` or `defer`, or use `<Script>` from `next/script`.

## Examples

### JavaScript

#### Don't

```js
const s = <script src="/a.js" />;
```

#### Fixed (defer)

```js
const s = <script src="/a.js" defer />;
```

#### Fixed (async)

```js
const s = <script src="/a.js" async />;
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

#### Fixed (defer)

```jsx
const s = <script src="/a.js" defer />;
```

#### Fixed (async)

```jsx
const s = <script src="/a.js" async />;
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

#### Fixed (defer)

```tsx
const s = <script src="/a.js" defer />;
```

#### Fixed (async)

```tsx
const s = <script src="/a.js" async />;
```

#### Do

```tsx
const s = <script src="/a.js" defer />;
```
