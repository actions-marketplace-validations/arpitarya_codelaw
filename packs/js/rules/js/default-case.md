---
id: js/default-case
title: "A switch has a default"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "what the default does with an unmatched value is chosen: a quiet `break` or a thrown error"
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
  - pattern: switch ($D) { $$$C }
  - not:
      has:
        field: body
        has: {kind: switch_default}
  - not:
      has:
        field: body
        has: {kind: comment, regex: '(?i)no default'}
message: Add a `default` clause
fix:
  choices:
    - label: break
      default: true
      fix: "switch ($D) {\n  $$$C\n  default:\n    break;\n}"
    - label: throw
      fix: "switch ($D) {\n  $$$C\n  default:\n    throw new Error(`Unexpected ${$D}`);\n}"
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

#### Fixed (break)

```js
switch (x) {
  case 1:
    a();
  default:
    break;
}
```

#### Fixed (throw)

```js
switch (x) {
  case 1:
    a();
  default:
    throw new Error(`Unexpected ${x}`);
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

#### Fixed (break)

```jsx
switch (x) {
  case 1:
    a();
  default:
    break;
}
```

#### Fixed (throw)

```jsx
switch (x) {
  case 1:
    a();
  default:
    throw new Error(`Unexpected ${x}`);
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

#### Fixed (break)

```ts
switch (x) {
  case 1:
    a();
  default:
    break;
}
```

#### Fixed (throw)

```ts
switch (x) {
  case 1:
    a();
  default:
    throw new Error(`Unexpected ${x}`);
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

#### Fixed (break)

```tsx
switch (x) {
  case 1:
    a();
  default:
    break;
}
```

#### Fixed (throw)

```tsx
switch (x) {
  case 1:
    a();
  default:
    throw new Error(`Unexpected ${x}`);
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
