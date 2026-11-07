---
id: js/prefer-scoped-selector
title: "Start an element's selector with :scope when it has a combinator"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "with `:scope` the selector no longer matches through ancestors outside the element; that is the intent, but it changes which elements are found"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-scoped-selector]
since: 0.5.0
---

`element.querySelector('.a .b')` matches `.b` inside the element whose `.a` may be an ancestor *outside* it: only the last part is limited to the subtree. With a combinator (a space, `>`, `+` or `~`) the selector starts with `:scope` so every part is inside. A query on `document`, a `shadowRoot` or a fragment is left — `:scope` would match nothing there — and so are selector lists, which need it on each item.

**Why:** The surprising match shows up only when the page around the element changes.

```codelaw
match:
  kind: call_expression
  has: { field: function, kind: member_expression, all: [{ has: { field: property, regex: '^querySelector(All)?$' } }, { has: { field: object, pattern: $E } }] }
  all:
  - has: { field: arguments, has: { nthChild: 1, kind: string, regex: '^["'']\s*[^\s:,>+~][^,]*?(\s|>|\+|~)\s*[^\s,]', not: { any: [{ regex: '^["'']\s*:scope' }, { regex: ',' }] }, pattern: $SEL } }
constraints:
  E: { not: { regex: '(^|\.)(document|shadowRoot|ownerDocument|fragment|content)$' } }
message: "Begin this selector with `:scope`"
fix:
  - edit-text: { target: $SEL, pattern: '^(["''])\s*', replace: '$1:scope ' }
```

## Examples

### JavaScript

#### Don't

```js
const inner = card.querySelectorAll(".outer .inner");
```

#### Do

```js
const inner = card.querySelectorAll(":scope .outer .inner");
```


### JSX

#### Don't

```jsx
const inner = card.querySelectorAll(".outer .inner");
```

#### Do

```jsx
const inner = card.querySelectorAll(":scope .outer .inner");
```


### TypeScript

#### Don't

```ts
const inner = card.querySelectorAll(".outer .inner");
```

#### Do

```ts
const inner = card.querySelectorAll(":scope .outer .inner");
```


### TSX

#### Don't

```tsx
const inner = card.querySelectorAll(".outer .inner");
```

#### Do

```tsx
const inner = card.querySelectorAll(":scope .outer .inner");
```
