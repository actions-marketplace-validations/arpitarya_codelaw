---
id: js/prefer-event-target
title: "EventTarget rather than EventEmitter"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "`on`/`emit` become `addEventListener`/`dispatchEvent` for every user"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-event-target]
since: 0.5.0
---

A class does not extend `EventEmitter`.

**Why:** `EventTarget` is standard in browsers, Node, Deno and workers.

```codelaw

match:
  kind: class_heritage
  regex: '\bEventEmitter\b'
message: "Extend `EventTarget`, which runs everywhere, rather than Node's `EventEmitter`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
class Store extends EventEmitter {}
```

#### Do

```js
class Store extends EventTarget {}
```

### JSX

#### Don't

```jsx
class Store extends EventEmitter {}
```

#### Do

```jsx
class Store extends EventTarget {}
```

### TypeScript

#### Don't

```ts
class Store extends EventEmitter {}
```

#### Do

```ts
class Store extends EventTarget {}
```

### TSX

#### Don't

```tsx
class Store extends EventEmitter {}
```

#### Do

```tsx
class Store extends EventTarget {}
```
