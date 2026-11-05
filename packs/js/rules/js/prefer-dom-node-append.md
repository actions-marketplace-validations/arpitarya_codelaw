---
id: js/prefer-dom-node-append
title: "append() rather than appendChild()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`appendChild` returns the node and `append` returns nothing"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-dom-node-append]
since: 0.5.0
---

`appendChild` is written `append` when its return value is not used.

**Why:** `append` also takes strings and several nodes.

```codelaw
match:
  pattern: "$P.appendChild($N)"
  inside: { kind: expression_statement }
message: "Use `.append()`"
fix: "$P.append($N)"
```

## Examples

### JavaScript

#### Don't

```js
list.appendChild(item);
```

#### Do

```js
list.append(item);
```

### JSX

#### Don't

```jsx
list.appendChild(item);
```

#### Do

```jsx
list.append(item);
```

### TypeScript

#### Don't

```ts
list.appendChild(item);
```

#### Do

```ts
list.append(item);
```

### TSX

#### Don't

```tsx
list.appendChild(item);
```

#### Do

```tsx
list.append(item);
```
