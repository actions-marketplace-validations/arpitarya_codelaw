---
id: ts/ban-tslint-comment
title: "No tslint comments"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/ban-tslint-comment]
since: 0.5.0
---

`// tslint:` directives are removed.

**Why:** TSLint is retired; its directives do nothing and mislead.

```codelaw
match:
  kind: comment
  regex: '^(//|/\*)\s*tslint:(enable|disable)'
message: "Remove this TSLint directive; TSLint no longer runs"
fix:
  - remove
```

## Examples

### TypeScript

#### Don't

```ts
// tslint:disable-next-line
const a = 1;
```

#### Do

```ts
const a = 1;
```

#### Fixed

```ts
const a = 1;
```

### TSX

#### Don't

```tsx
// tslint:disable-next-line
const a = 1;
```

#### Do

```tsx
const a = 1;
```

#### Fixed

```tsx
const a = 1;
```
