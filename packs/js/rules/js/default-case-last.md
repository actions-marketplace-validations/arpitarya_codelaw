---
id: js/default-case-last
title: "default comes last"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving a clause can change fall-through behaviour"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint/default-case-last, eslint-plugin-sonarjs/prefer-default-last]
since: 0.5.0
---

The `default` clause of a `switch` is its last clause.

**Why:** A `default` in the middle is easy to miss and makes fall-through order hard to follow.

```codelaw
match:
  kind: switch_default
  precedes: { kind: switch_case }
message: "Move `default` to the end of the `switch`"
```

## How to fix

1. Move the `default` clause to the end, checking any fall-through into or out of it.

## Examples

### JavaScript

#### Don't

```js
switch (x) {
  default:
    a();
    break;
  case 1:
    b();
}
```

#### Do

```js
switch (x) {
  case 1:
    b();
    break;
  default:
    a();
}
```

### JSX

#### Don't

```jsx
switch (x) {
  default:
    a();
    break;
  case 1:
    b();
}
```

#### Do

```jsx
switch (x) {
  case 1:
    b();
    break;
  default:
    a();
}
```

### TypeScript

#### Don't

```ts
switch (x) {
  default:
    a();
    break;
  case 1:
    b();
}
```

#### Do

```ts
switch (x) {
  case 1:
    b();
    break;
  default:
    a();
}
```

### TSX

#### Don't

```tsx
switch (x) {
  default:
    a();
    break;
  case 1:
    b();
}
```

#### Do

```tsx
switch (x) {
  case 1:
    b();
    break;
  default:
    a();
}
```
