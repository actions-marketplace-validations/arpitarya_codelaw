---
id: js/require-post-message-target-origin
title: "postMessage names its target origin"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the origin to trust is the author's choice"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/require-post-message-target-origin]
since: 0.5.0
---

`window.postMessage` is called with its target-origin argument.

**Why:** Without it, `"*"` is implied in some engines and the message can reach any origin.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: ^(window|self|globalThis|parent|top|opener)\.postMessage$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
      not:
        has:
          nthChild: 2
message: Pass the target origin to `postMessage`
```

## How to fix

1. Pass the exact origin the message is for, such as `location.origin` or the iframe's origin.

## Examples

### JavaScript

#### Don't

```js
window.postMessage(data);
```

#### Do

```js
window.postMessage(data, location.origin);
```

### JSX

#### Don't

```jsx
window.postMessage(data);
```

#### Do

```jsx
window.postMessage(data, location.origin);
```

### TypeScript

#### Don't

```ts
window.postMessage(data);
```

#### Do

```ts
window.postMessage(data, location.origin);
```

### TSX

#### Don't

```tsx
window.postMessage(data);
```

#### Do

```tsx
window.postMessage(data, location.origin);
```
