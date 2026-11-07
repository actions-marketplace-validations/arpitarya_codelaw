---
id: js/dom-node-dataset
title: "Use dataset for data- attributes"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`.dataset` gives `undefined` where `getAttribute` gave `null`, and keys like `data-x-1` map to `x-1`; the author confirms the reads"
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
        pattern: $K
  - any:
    - { pattern: "$O.setAttribute($K, $V)", all: [{ pattern: $SET }] }
    - { pattern: "$O.getAttribute($K)", all: [{ pattern: $GET }] }
    - { pattern: "$O.removeAttribute($K)", all: [{ pattern: $DEL }] }
    - { pattern: "$O.hasAttribute($K)", all: [{ pattern: $HAS }] }
    - { regex: '' }
transform:
  N: { substring: { source: $K, startChar: 6, endChar: -1 } }
  C: { convert: { source: $N, toCase: camelCase, separatedBy: [dash] } }
message: Read and write `data-*` attributes through `.dataset`
fix:
  - replace: { target: $SET, with: "$O.dataset.$C = $V" }
  - replace: { target: $GET, with: "$O.dataset.$C" }
  - replace: { target: $DEL, with: "delete $O.dataset.$C" }
  - replace: { target: $HAS, with: "'$C' in $O.dataset" }
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
