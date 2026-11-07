---
id: js/prefer-dom-node-html-methods
title: "getHTML() rather than reading innerHTML"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-dom-node-html-methods]
since: 0.5.0
---

An element's markup is read with `element.getHTML()`, not `element.innerHTML`. With no options the two give the same string. Writes to `innerHTML` are left, as upstream leaves them by default: `setHTML()`, which sanitizes, is not in every browser yet.

**Why:** `getHTML()` is the method form, and takes options (`serializableShadowRoots`) the property cannot.

```codelaw
match:
  kind: member_expression
  pattern: "$E.innerHTML"
  not:
    any:
      - inside: { kind: assignment_expression, field: left }
      - inside: { kind: augmented_assignment_expression, field: left }
      - inside: { kind: update_expression }
      - inside: { kind: unary_expression, regex: '^delete\b' }
      - has: { kind: optional_chain }
constraints:
  E: { not: { kind: super } }
message: "Read the markup with `getHTML()`"
fix: "$E.getHTML()"
```

## Examples

### JavaScript

#### Don't

```js
const markup = panel.innerHTML;
```

#### Do

```js
const markup = panel.getHTML();
```


### JSX

#### Don't

```jsx
const markup = panel.innerHTML;
```

#### Do

```jsx
const markup = panel.getHTML();
```


### TypeScript

#### Don't

```ts
const markup = panel.innerHTML;
```

#### Do

```ts
const markup = panel.getHTML();
```


### TSX

#### Don't

```tsx
const markup = panel.innerHTML;
```

#### Do

```tsx
const markup = panel.getHTML();
```
