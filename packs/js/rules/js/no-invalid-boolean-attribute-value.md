---
id: js/no-invalid-boolean-attribute-value
title: "No \"false\" value for a boolean attribute"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "removing the attribute is what \"false\" usually means, but the author confirms"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-boolean-attribute-value]
since: 0.5.0
---

A boolean attribute (`disabled`, `hidden`, `checked`, `required`, …) is not set to the string `"false"`: any value turns it on. It is removed instead.

**Why:** `setAttribute("disabled", "false")` disables the element.

```codelaw
match:
  pattern: "$E.setAttribute($N, $V)"
constraints:
  N: { regex: '^["''](allowfullscreen|async|autofocus|autoplay|checked|controls|default|defer|disabled|formnovalidate|hidden|inert|ismap|itemscope|loop|multiple|muted|nomodule|novalidate|open|playsinline|readonly|required|reversed|selected)["'']$' }
  V: { regex: '^["''`]false["''`]$' }
message: "Any value turns `$N` on; remove the attribute instead"
fix: "$E.removeAttribute($N)"
```

## Examples

### JavaScript

#### Don't

```js
button.setAttribute("disabled", "false");
```

#### Do

```js
button.removeAttribute("disabled");
```


### JSX

#### Don't

```jsx
button.setAttribute("disabled", "false");
```

#### Do

```jsx
button.removeAttribute("disabled");
```


### TypeScript

#### Don't

```ts
button.setAttribute("disabled", "false");
```

#### Do

```ts
button.removeAttribute("disabled");
```


### TSX

#### Don't

```tsx
button.setAttribute("disabled", "false");
```

#### Do

```tsx
button.removeAttribute("disabled");
```
