---
id: js/prefer-add-event-listener
title: "addEventListener rather than an on-property"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the listener can no longer be replaced by a later assignment, which other code may rely on"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-add-event-listener]
since: 0.5.0
---

Event handlers are added with `addEventListener`, not by assigning an `on…` property.

**Why:** An assignment replaces whatever handler was there before; listeners stack.

```codelaw
match:
  kind: assignment_expression
  has:
    field: left
    kind: member_expression
    has:
      field: property
      regex: ^on(click|change|input|submit|load|error|keydown|keyup|keypress|mousedown|mouseup|mousemove|mouseover|mouseout|focus|blur|scroll|resize|message|close|open)$
  all:
  - has:
      field: right
      not:
        kind: 'null'
message: Use `addEventListener` rather than assigning an `on…` property
```

## How to fix

1. Write `el.addEventListener("click", handler)`; to remove it later keep the handler and call `removeEventListener`.

## Examples

### JavaScript

#### Don't

```js
button.onclick = save;
```

#### Do

```js
button.addEventListener("click", save);
```

### JSX

#### Don't

```jsx
button.onclick = save;
```

#### Do

```jsx
button.addEventListener("click", save);
```

### TypeScript

#### Don't

```ts
button.onclick = save;
```

#### Do

```ts
button.addEventListener("click", save);
```

### TSX

#### Don't

```tsx
button.onclick = save;
```

#### Do

```tsx
button.addEventListener("click", save);
```
