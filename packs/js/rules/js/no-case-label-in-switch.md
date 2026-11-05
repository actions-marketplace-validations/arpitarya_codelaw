---
id: js/no-case-label-in-switch
title: "No label inside a case"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether a case or a label was meant is the author's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-case-label-in-switch]
since: 0.5.0
---

No labelled statement sits directly in a `case` body.

**Why:** A label like `other:` reads as a mistyped `case other:`.

```codelaw

match:
  kind: labeled_statement
  inside: { any: [{kind: switch_case}, {kind: switch_default}] }
message: "A label in a `case` looks like another `case`; remove it"
```

## How to fix

1. Remove the label, or turn it into the `case` that was meant.

## Examples

### JavaScript

#### Don't

```js
switch (k) {
  case 1:
    other: run();
    break;
}
```

#### Do

```js
switch (k) {
  case 1:
    run();
    break;
}
```

### JSX

#### Don't

```jsx
switch (k) {
  case 1:
    other: run();
    break;
}
```

#### Do

```jsx
switch (k) {
  case 1:
    run();
    break;
}
```

### TypeScript

#### Don't

```ts
switch (k) {
  case 1:
    other: run();
    break;
}
```

#### Do

```ts
switch (k) {
  case 1:
    run();
    break;
}
```

### TSX

#### Don't

```tsx
switch (k) {
  case 1:
    other: run();
    break;
}
```

#### Do

```tsx
switch (k) {
  case 1:
    run();
    break;
}
```
