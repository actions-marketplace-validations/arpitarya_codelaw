---
id: js/default-case-last
title: "default comes last"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "moved automatically when no clause falls into another across the move: `default`, the clause before it and the last clause each end with `break`, `return`, `throw` or `continue`; otherwise moving it changes which statements run, and it is moved by hand"
stability: preview
ported_from: [eslint/default-case-last, eslint-plugin-sonarjs/prefer-default-last]
since: 0.5.0
---

The `default` clause of a `switch` is its last clause.

**Why:** A `default` in the middle is easy to miss and makes fall-through order hard to follow.

```codelaw
utils:
  jump: { any: [{ kind: break_statement }, { kind: return_statement }, { kind: throw_statement }, { kind: continue_statement }] }
match:
  kind: switch_default
  precedes: { kind: switch_case }
  any:
    - all:
        - has: { nthChild: { position: 1, reverse: true }, matches: jump }
        - any:
            - not: { follows: { any: [{ kind: switch_case }, { kind: switch_default }] } }
            - follows: { kind: switch_case, has: { nthChild: { position: 1, reverse: true }, matches: jump } }
        - precedes:
            stopBy: end
            kind: switch_case
            not: { precedes: { any: [{ kind: switch_case }, { kind: switch_default }] } }
            has: { nthChild: { position: 1, reverse: true }, matches: jump }
            pattern: $LAST
    - regex: ''
message: "Move `default` to the end of the `switch`"
fix:
  - move: { after: $LAST }
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
    break;
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
    break;
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
    break;
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
    break;
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
    break;
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
    break;
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
    break;
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
    break;
}
```
