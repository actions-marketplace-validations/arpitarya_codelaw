---
id: js/prefer-add-event-listener-options
title: "An options object rather than a capture boolean"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-add-event-listener-options]
since: 0.5.0
---

`addEventListener` and `removeEventListener` take `{ capture: true }`, not a bare `true`, as their third argument; a bare `false` is the default and is dropped.

**Why:** A bare boolean doesn't say what it switches on.

```codelaw
match:
  any:
    - pattern: "$E.$M($T, $F, true)"
      all: [{ pattern: $ON }]
    - pattern: "$E.$M($T, $F, false)"
      all: [{ pattern: $OFF }]
constraints:
  M: { regex: '^(add|remove)EventListener$' }
message: "Pass an options object"
fix:
  - replace: { target: $ON, with: "$E.$M($T, $F, { capture: true })" }
  - replace: { target: $OFF, with: "$E.$M($T, $F)" }
```

## Examples

### JavaScript

#### Don't

```js
document.addEventListener("click", onClick, true);
```

#### Do

```js
document.addEventListener("click", onClick, { capture: true });
```


### JSX

#### Don't

```jsx
document.addEventListener("click", onClick, true);
```

#### Do

```jsx
document.addEventListener("click", onClick, { capture: true });
```


### TypeScript

#### Don't

```ts
document.addEventListener("click", onClick, true);
```

#### Do

```ts
document.addEventListener("click", onClick, { capture: true });
```


### TSX

#### Don't

```tsx
document.addEventListener("click", onClick, true);
```

#### Do

```tsx
document.addEventListener("click", onClick, { capture: true });
```
