---
id: js/explicit-timer-delay
title: "Give setTimeout and setInterval a delay"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/explicit-timer-delay]
since: 0.5.0
---

`setTimeout` and `setInterval` are called with an explicit delay.

**Why:** A missing delay means 0, which reads as a forgotten argument.

```codelaw

match:
  pattern: $T($F)
constraints:
  T: { regex: '^(setTimeout|setInterval)$' }
  F: { not: { kind: spread_element } }
message: "Pass the delay to `$T` explicitly"
fix: "$T($F, 0)"
```

## Examples

### JavaScript

#### Don't

```js
setTimeout(flush);
```

#### Do

```js
setTimeout(flush, 0);
```

### JSX

#### Don't

```jsx
setTimeout(flush);
```

#### Do

```jsx
setTimeout(flush, 0);
```

### TypeScript

#### Don't

```ts
setTimeout(flush);
```

#### Do

```ts
setTimeout(flush, 0);
```

### TSX

#### Don't

```tsx
setTimeout(flush);
```

#### Do

```tsx
setTimeout(flush, 0);
```
