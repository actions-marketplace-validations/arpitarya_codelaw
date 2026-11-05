---
id: js/require-passive-events
title: "Touch and wheel listeners are passive"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a passive listener can no longer call `preventDefault()`"
stability: preview
ported_from: [eslint-plugin-unicorn/require-passive-events]
since: 0.5.0
---

A `touch*` or `wheel` listener is added with `{ passive: true }`.

**Why:** The browser otherwise waits for the listener before scrolling, which makes the page feel sluggish.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: \.addEventListener$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        kind: string
        regex: ^.(touchstart|touchmove|touchenter|touchend|touchleave|wheel|mousewheel).$
      not:
        has:
          nthChild: 3
      all:
      - has:
          nthChild: 2
          pattern: $F
message: 'Mark this listener `{ passive: true }` so scrolling does not wait for it'
fix:
- insert-after:
    target: $F
    text: ', { passive: true }'
```

## Examples

### JavaScript

#### Don't

```js
el.addEventListener("wheel", onWheel);
```

#### Do

```js
el.addEventListener("wheel", onWheel, { passive: true });
```

### JSX

#### Don't

```jsx
el.addEventListener("wheel", onWheel);
```

#### Do

```jsx
el.addEventListener("wheel", onWheel, { passive: true });
```

### TypeScript

#### Don't

```ts
el.addEventListener("wheel", onWheel);
```

#### Do

```ts
el.addEventListener("wheel", onWheel, { passive: true });
```

### TSX

#### Don't

```tsx
el.addEventListener("wheel", onWheel);
```

#### Do

```tsx
el.addEventListener("wheel", onWheel, { passive: true });
```
