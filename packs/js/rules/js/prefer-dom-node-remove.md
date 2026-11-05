---
id: js/prefer-dom-node-remove
title: "node.remove() rather than parent.removeChild(node)"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`removeChild` throws if the node is not a child of that parent; `remove()` does not"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-dom-node-remove]
since: 0.5.0
---

`parent.removeChild(node)` is written `node.remove()`.

**Why:** `remove()` needs no reference to the parent.

```codelaw
match:
  pattern: "$P.removeChild($N)"
message: "Use `.remove()`"
fix: "$N.remove()"
```

## Examples

### JavaScript

#### Don't

```js
el.parentNode.removeChild(el);
```

#### Do

```js
el.remove();
```

### JSX

#### Don't

```jsx
el.parentNode.removeChild(el);
```

#### Do

```jsx
el.remove();
```

### TypeScript

#### Don't

```ts
el.parentNode.removeChild(el);
```

#### Do

```ts
el.remove();
```

### TSX

#### Don't

```tsx
el.parentNode.removeChild(el);
```

#### Do

```tsx
el.remove();
```
