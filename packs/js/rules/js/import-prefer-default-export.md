---
id: js/import-prefer-default-export
title: "A module with one export makes it the default"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "turning a named export into a default changes every importer"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/prefer-default-export]
since: 0.5.0
---

A module whose only export is one named value exports it as `default`. Type-only exports and re-exports are left alone.

**Why:** A project that picks this style gives each one-thing module the obvious import. Off unless a project opts in.

```codelaw
utils:
  named:
    kind: export_statement
    not:
      any:
        - regex: '^export\s+(default|type|\*)\b'
        - has: { field: source }
        - has: { field: declaration, any: [{ kind: type_alias_declaration }, { kind: interface_declaration }] }
match:
  matches: named
  not:
    any:
      - follows: { stopBy: end, kind: export_statement }
      - precedes: { stopBy: end, kind: export_statement }
      - has: { field: declaration, kind: lexical_declaration, has: { kind: variable_declarator, nthChild: 2 } }
      - has: { kind: export_clause, has: { kind: export_specifier, nthChild: 2 } }
message: "The module's only export can be its default"
```

## How to fix

1. Write `export default` and change `import { x }` to `import x` where it is used.

## Examples

### JavaScript

#### Don't

```js
export function slugify(text) {
  return text.toLowerCase();
}
```

#### Do

```js
export default function slugify(text) {
  return text.toLowerCase();
}
```


### JSX

#### Don't

```jsx
export function slugify(text) {
  return text.toLowerCase();
}
```

#### Do

```jsx
export default function slugify(text) {
  return text.toLowerCase();
}
```


### TypeScript

#### Don't

```ts
export function slugify(text) {
  return text.toLowerCase();
}
```

#### Do

```ts
export default function slugify(text) {
  return text.toLowerCase();
}
```


### TSX

#### Don't

```tsx
export function slugify(text) {
  return text.toLowerCase();
}
```

#### Do

```tsx
export default function slugify(text) {
  return text.toLowerCase();
}
```
