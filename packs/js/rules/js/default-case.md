---
id: js/default-case
title: "A switch has a default"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the default should do is the author's"
fix_pending: "suggest-choices"
stability: preview
ported_from: [eslint/default-case]
since: 0.5.0
---

A `switch` has a `default` clause, or a `// no default` comment at its end.

**Why:** Without one, a value no case handles passes silently.

```codelaw
match:
  kind: switch_statement
  all:
  - not:
      has:
        field: body
        has: {kind: switch_default}
  - not:
      has:
        field: body
        has: {kind: comment, regex: '(?i)no default'}
message: Add a `default` clause
```

## How to fix

1. Add `default:` that handles or reports the unexpected value, or a `// no default` comment.

## Examples

### JavaScript

#### Don't

```js
switch (x) {
  case 1:
    a();
}
```

#### Do

```js
switch (x) {
  case 1:
    a();
    break;
  default:
    fail(x);
}
```

### JSX

#### Don't

```jsx
switch (x) {
  case 1:
    a();
}
```

#### Do

```jsx
switch (x) {
  case 1:
    a();
    break;
  default:
    fail(x);
}
```

### TypeScript

#### Don't

```ts
switch (x) {
  case 1:
    a();
}
```

#### Do

```ts
switch (x) {
  case 1:
    a();
    break;
  default:
    fail(x);
}
```

### TSX

#### Don't

```tsx
switch (x) {
  case 1:
    a();
}
```

#### Do

```tsx
switch (x) {
  case 1:
    a();
    break;
  default:
    fail(x);
}
```
