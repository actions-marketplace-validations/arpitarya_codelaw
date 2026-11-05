---
id: js/no-useless-switch-case
title: "No empty case before default"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the case may be there to document a value that is expected"
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-switch-case]
since: 0.5.0
---

An empty `case` directly before `default` is removed.

**Why:** It does what `default` does, so it only suggests a difference.

```codelaw

match:
  kind: switch_case
  not: { has: { field: body, regex: "." } }
  precedes: { kind: switch_default }
message: "This case falls through to `default`; it adds nothing"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
switch (kind) {
  case "a":
    run();
    break;
  case "b":
  default:
    skip();
}
```

#### Do

```js
switch (kind) {
  case "a":
    run();
    break;
  default:
    skip();
}
```

### JSX

#### Don't

```jsx
switch (kind) {
  case "a":
    run();
    break;
  case "b":
  default:
    skip();
}
```

#### Do

```jsx
switch (kind) {
  case "a":
    run();
    break;
  default:
    skip();
}
```

### TypeScript

#### Don't

```ts
switch (kind) {
  case "a":
    run();
    break;
  case "b":
  default:
    skip();
}
```

#### Do

```ts
switch (kind) {
  case "a":
    run();
    break;
  default:
    skip();
}
```

### TSX

#### Don't

```tsx
switch (kind) {
  case "a":
    run();
    break;
  case "b":
  default:
    skip();
}
```

#### Do

```tsx
switch (kind) {
  case "a":
    run();
    break;
  default:
    skip();
}
```
