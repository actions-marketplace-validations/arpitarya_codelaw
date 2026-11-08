---
id: js/no-prevent-default-in-passive-listener
title: "No preventDefault() in a passive listener"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "either the listener should not be passive or the call should go; making it not passive is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-prevent-default-in-passive-listener]
since: 0.5.0
---

A listener added with `{ passive: true }` does not call `preventDefault()`: the browser ignores the call and warns.

**Why:** Passive means the listener promises not to cancel the event, so the cancel silently does nothing.

```codelaw
match:
  pattern: "$E.addEventListener($T, $F, $O)"
  has:
    field: arguments
    all:
      - has: { kind: object, pattern: $O, has: { kind: pair, regex: '^passive\s*:\s*true$' } }
      - has: { stopBy: end, pattern: "$X.preventDefault()" }
message: "`preventDefault()` does nothing in a passive listener"
fix:
  - edit-text: { target: $O, pattern: 'passive(\s*):(\s*)true', replace: 'passive${1}:${2}false' }
```

## Examples

### JavaScript

#### Don't

```js
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: true });
```

#### Do

```js
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: false });
```


### JSX

#### Don't

```jsx
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: true });
```

#### Do

```jsx
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: false });
```


### TypeScript

#### Don't

```ts
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: true });
```

#### Do

```ts
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: false });
```


### TSX

#### Don't

```tsx
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: true });
```

#### Do

```tsx
el.addEventListener("touchmove", (e) => { e.preventDefault(); }, { passive: false });
```
