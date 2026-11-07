---
id: js/prefer-keyboard-event-key
title: "event.key rather than keyCode"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`key` names the key as typed, so a code compared with a number is rewritten only for the named keys (Enter, Escape, the arrows, …) whose `key` does not change with the layout or Shift; letters, digits and a `keyCode` read any other way are rewritten by hand"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-keyboard-event-key]
since: 0.5.0
---

`keyCode`, `charCode` and `which` are not read from an event; `key` is.

**Why:** The numeric codes are deprecated and differ between layouts.

```codelaw
match:
  pattern: "$E.keyCode"
  any:
    - all:
        - has: { field: property, pattern: $PROP }
        - inside: { kind: binary_expression, all: [{ has: { field: operator, regex: '^(===?|!==?)$' } }, { has: { kind: number, pattern: $N } }] }
    - regex: ''
message: "Use `.key`"
fix:
  - replace: { target: $PROP, with: key }
  - replace: { target: $N, with: '"$N|map(8:Backspace, 9:Tab, 13:Enter, 16:Shift, 17:Control, 18:Alt, 27:Escape, 32: , 33:PageUp, 34:PageDown, 35:End, 36:Home, 37:ArrowLeft, 38:ArrowUp, 39:ArrowRight, 40:ArrowDown, 45:Insert, 46:Delete)"' }
```

## How to fix

1. Read `event.key` and compare with the key name (`"Enter"` for 13).

## Examples

### JavaScript

#### Don't

```js
if (event.keyCode === 13) {
  submit();
}
```

#### Do

```js
if (event.key === "Enter") {
  submit();
}
```

### JSX

#### Don't

```jsx
if (event.keyCode === 13) {
  submit();
}
```

#### Do

```jsx
if (event.key === "Enter") {
  submit();
}
```

### TypeScript

#### Don't

```ts
if (event.keyCode === 13) {
  submit();
}
```

#### Do

```ts
if (event.key === "Enter") {
  submit();
}
```

### TSX

#### Don't

```tsx
if (event.keyCode === 13) {
  submit();
}
```

#### Do

```tsx
if (event.key === "Enter") {
  submit();
}
```
