---
id: ts/no-useless-empty-export
title: "No export {} in a module"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [typescript-eslint/no-useless-empty-export]
since: 0.5.0
---

`export {}` is not written in a file that already has an import or export.

**Why:** It only turns a script into a module, which the other import or export already does.

```codelaw
match:
  kind: export_statement
  regex: '^export\s*\{\s*\};?$'
  inside:
    kind: program
    has:
      any:
        - kind: import_statement
        - { kind: export_statement, not: { regex: '^export\s*\{\s*\};?$' } }
message: "This `export {}` does nothing; the file is already a module"
fix:
  - remove
```

## Examples

### TypeScript

#### Don't

```ts
export const a = 1;
export {};
```

#### Do

```ts
export const a = 1;
```

#### Fixed

```ts
export const a = 1;
```

### TSX

#### Don't

```tsx
export const a = 1;
export {};
```

#### Do

```tsx
export const a = 1;
```

#### Fixed

```tsx
export const a = 1;
```
