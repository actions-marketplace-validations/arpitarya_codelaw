---
id: js/prefer-modern-dom-apis
title: "before/replaceWith rather than insertBefore/replaceChild"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`replaceChild` returns the old node and throws when it is not a child of that parent"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-modern-dom-apis]
since: 0.5.0
---

`parent.replaceChild(new, old)` is written `old.replaceWith(new)`. Upstream also covers `insertBefore` and `insertAdjacent*`; this port covers `replaceChild`.

**Why:** The modern method needs no reference to the parent and reads in the order things happen.

```codelaw

match:
  pattern: $P.replaceChild($N, $O)
message: "Use `$O.replaceWith($N)`"
fix: "$O.replaceWith($N)"
```

## Examples

### JavaScript

#### Don't

```js
list.replaceChild(fresh, stale);
```

#### Do

```js
stale.replaceWith(fresh);
```

### JSX

#### Don't

```jsx
list.replaceChild(fresh, stale);
```

#### Do

```jsx
stale.replaceWith(fresh);
```

### TypeScript

#### Don't

```ts
list.replaceChild(fresh, stale);
```

#### Do

```ts
stale.replaceWith(fresh);
```

### TSX

#### Don't

```tsx
list.replaceChild(fresh, stale);
```

#### Do

```tsx
stale.replaceWith(fresh);
```
