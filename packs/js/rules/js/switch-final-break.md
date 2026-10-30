---
id: js/switch-final-break
title: "No break at the end of the last switch clause"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [tslint/switch-final-break]
since: 0.5.0
---

The last clause of a `switch` does not end in a bare `break;`: control leaves the `switch` there anyway. A labelled `break` is left alone.

**Why:** The last `break` does nothing, and one less line to read is one less to wonder about.

```codelaw
match:
  kind: break_statement
  regex: '^break\s*;?$'
  nthChild: { position: 1, reverse: true }
  inside:
    any: [{ kind: switch_case }, { kind: switch_default }]
    nthChild: { position: 1, reverse: true }
message: "This `break` ends the last clause; drop it"
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
  default:
    stop();
    break;
}
```

#### Do

```js
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
}
```

### JSX

#### Don't

```jsx
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
    break;
}
```

#### Do

```jsx
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
}
```

### TypeScript

#### Don't

```ts
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
    break;
}
```

#### Do

```ts
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
}
```

### TSX

#### Don't

```tsx
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
    break;
}
```

#### Do

```tsx
switch (kind) {
  case "a":
    run();
    break;
  default:
    stop();
}
```
