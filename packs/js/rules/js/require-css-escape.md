---
id: js/require-css-escape
title: "Escape values in a selector"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which parts are values is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/require-css-escape]
since: 0.5.0
---

A value joined into a selector string passed to `querySelector`, `querySelectorAll`, `closest` or `matches` goes through `CSS.escape`.

**Why:** An id or class with a special character makes the selector invalid or match something else.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: \.(querySelector|querySelectorAll|closest|matches)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        any:
        - kind: binary_expression
          has:
            kind: identifier
        - kind: template_string
          has:
            kind: template_substitution
            not:
              regex: CSS\.escape
message: Escape the value with `CSS.escape()` before putting it in a selector
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const row = table.querySelector("#" + id);
```

#### Do

```js
const row = table.querySelector("#" + CSS.escape(id));
```

### JSX

#### Don't

```jsx
const row = table.querySelector("#" + id);
```

#### Do

```jsx
const row = table.querySelector("#" + CSS.escape(id));
```

### TypeScript

#### Don't

```ts
const row = table.querySelector("#" + id);
```

#### Do

```ts
const row = table.querySelector("#" + CSS.escape(id));
```

### TSX

#### Don't

```tsx
const row = table.querySelector("#" + id);
```

#### Do

```tsx
const row = table.querySelector("#" + CSS.escape(id));
```
