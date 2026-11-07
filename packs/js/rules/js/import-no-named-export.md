---
id: js/import-no-named-export
title: "No named exports"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning named exports into a default changes every importer"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/no-named-export]
since: 0.5.0
---

A module exports only a default (`export default`); named exports are not used. Type-only exports are left alone.

**Why:** A project that picks default-only exports gives each module one entry point. Off unless a project opts in.

```codelaw
match:
  kind: export_statement
  not:
    any:
      - regex: '^export\s+default\b'
      - regex: '^export\s+type\b'
      - has: { field: declaration, any: [{ kind: type_alias_declaration }, { kind: interface_declaration }] }
message: "Export this as the module's default"
```

## How to fix

1. Make the module's one export its default, and update the imports.

## Examples

### JavaScript

#### Don't

```js
export function parse(text) {}
```

#### Do

```js
export default function parse(text) {}
```


### JSX

#### Don't

```jsx
export function parse(text) {}
```

#### Do

```jsx
export default function parse(text) {}
```


### TypeScript

#### Don't

```ts
export function parse(text) {}
```

#### Do

```ts
export default function parse(text) {}
```


### TSX

#### Don't

```tsx
export function parse(text) {}
```

#### Do

```tsx
export default function parse(text) {}
```
