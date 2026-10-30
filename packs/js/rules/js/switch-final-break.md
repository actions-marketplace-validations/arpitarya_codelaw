---
id: js/switch-final-break
title: "No break at the end of the last case"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [tslint/switch-final-break]
since: 0.5.0
---

The last clause of a `switch` doesn't end in `break` (the upstream default, `"never"`). A labelled `break` is left alone.

**Why:** The `break` jumps to where control already goes next; it is noise in a construct that is already noisy.

```codelaw
match:
  kind: break_statement
  inside:
    any: [{kind: switch_case}, {kind: switch_default}]
    not: { precedes: { stopBy: end, any: [{kind: switch_case}, {kind: switch_default}] } }
  all:
    - not: { has: { kind: statement_identifier } }
    - not: { precedes: { stopBy: end, regex: ".", not: { kind: comment } } }
message: "This `break` ends the last clause anyway; remove it"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
    break;
}
```

#### Do

```js
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
}
```

### JSX

#### Don't

```jsx
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
    break;
}
```

#### Do

```jsx
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
}
```

### TypeScript

#### Don't

```ts
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
    break;
}
```

#### Do

```ts
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
}
```

### TSX

#### Don't

```tsx
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
    break;
}
```

#### Do

```tsx
switch (mode) {
  case "a":
    start();
    break;
  default:
    stop();
}
```
