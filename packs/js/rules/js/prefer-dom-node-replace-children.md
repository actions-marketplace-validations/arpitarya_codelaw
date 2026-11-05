---
id: js/prefer-dom-node-replace-children
title: "replaceChildren() rather than innerHTML = \"\""
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-dom-node-replace-children]
since: 0.5.0
---

An element is emptied with `replaceChildren()`, not by assigning `""` to `innerHTML` or `textContent`. This port covers `innerHTML`.

**Why:** `replaceChildren` says what it does and does not invoke the HTML parser.

```codelaw
match:
  pattern: "$E.innerHTML = \"\""
message: "Use `.replaceChildren()`"
fix: "$E.replaceChildren()"
```

## Examples

### JavaScript

#### Don't

```js
list.innerHTML = "";
```

#### Do

```js
list.replaceChildren();
```

### JSX

#### Don't

```jsx
list.innerHTML = "";
```

#### Do

```jsx
list.replaceChildren();
```

### TypeScript

#### Don't

```ts
list.innerHTML = "";
```

#### Do

```ts
list.replaceChildren();
```

### TSX

#### Don't

```tsx
list.innerHTML = "";
```

#### Do

```tsx
list.replaceChildren();
```
