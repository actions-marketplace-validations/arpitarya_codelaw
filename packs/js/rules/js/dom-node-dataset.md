---
id: js/dom-node-dataset
title: "Use dataset for data- attributes"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the camel-cased `dataset` key is computed from the attribute name, which a fix template cannot do"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-unicorn/dom-node-dataset]
since: 0.5.0
---

`data-*` attributes are read and written through the element's `dataset`, not `setAttribute`/`getAttribute`/`removeAttribute`/`hasAttribute`.

**Why:** `dataset` maps `data-user-id` to `userId` and back, and reads as property access.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    kind: member_expression
    has:
      field: property
      regex: ^(setAttribute|getAttribute|removeAttribute|hasAttribute)$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.data-[a-z0-9-]+.$
message: Read and write `data-*` attributes through `.dataset`
```

## How to fix

1. Replace `el.setAttribute("data-user-id", v)` with `el.dataset.userId = v`, `getAttribute` with a read, `removeAttribute` with `delete el.dataset.userId` and `hasAttribute` with `"userId" in el.dataset`.

## Examples

### JavaScript

#### Don't

```js
el.setAttribute("data-id", id);
```

#### Do

```js
el.dataset.id = id;
```

### JSX

#### Don't

```jsx
el.setAttribute("data-id", id);
```

#### Do

```jsx
el.dataset.id = id;
```

### TypeScript

#### Don't

```ts
el.setAttribute("data-id", id);
```

#### Do

```ts
el.dataset.id = id;
```

### TSX

#### Don't

```tsx
el.setAttribute("data-id", id);
```

#### Do

```tsx
el.dataset.id = id;
```
