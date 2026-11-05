---
id: js/prefer-keyboard-event-key
title: "event.key rather than keyCode"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the key name for a numeric code comes from a table"
fix_pending: "suggest; needs computed-edit"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-keyboard-event-key]
since: 0.5.0
---

`keyCode`, `charCode` and `which` are not read from an event; `key` is.

**Why:** The numeric codes are deprecated and differ between layouts.

```codelaw
match:
  pattern: "$E.keyCode"
message: "Use `.key`"
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
