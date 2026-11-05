---
id: ts/no-inferrable-types
title: "No annotation that the value already says"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/no-inferrable-types]
since: 0.5.0
---

A variable, parameter or property initialised with a literal does not also annotate the literal's type.

**Why:** `const n: number = 5` repeats what TypeScript infers.

```codelaw
match:
  kind: variable_declarator
  all:
  - has: {field: name, pattern: $N}
  - has: {field: value, pattern: $V}
  - any:
    - all:
      - has: {field: type, regex: ':\s*number$'}
      - has: {field: value, kind: number}
    - all:
      - has: {field: type, regex: ':\s*string$'}
      - has:
          field: value
          any:
          - {kind: string}
          - {kind: template_string}
    - all:
      - has: {field: type, regex: ':\s*boolean$'}
      - has:
          field: value
          any:
          - {kind: 'true'}
          - {kind: 'false'}
message: The annotation repeats what the value says; remove it
fix: $N = $V
```

## Examples

### TypeScript

#### Don't

```ts
const retries: number = 3;
```

#### Do

```ts
const retries = 3;
```

#### Fixed

```ts
const retries = 3;
```

### TSX

#### Don't

```tsx
const retries: number = 3;
```

#### Do

```tsx
const retries = 3;
```

#### Fixed

```tsx
const retries = 3;
```

## Differences from the source

Variables only; the source also covers parameters and properties.
