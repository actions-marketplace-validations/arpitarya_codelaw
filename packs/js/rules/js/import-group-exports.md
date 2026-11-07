---
id: js/import-group-exports
title: "Named exports in one export statement"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "gathering the exports moves declarations; the author arranges them"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-import/group-exports]
since: 0.5.0
---

A module's named exports are gathered in one `export { … }` statement rather than spread over several.

**Why:** One list shows the module's interface in one place. Off unless a project opts in.

```codelaw
utils:
  named:
    kind: export_statement
    not:
      any:
        - regex: '^export\s+(default|type|\*)\b'
        - has: { field: source }
match:
  matches: named
  follows: { stopBy: end, matches: named }
message: "Gather the named exports in one `export { … }`"
```

## How to fix

1. Declare the values without `export`, and list them in one `export { a, b }` at the end of the file.

## Examples

### JavaScript

#### Don't

```js
export const a = 1;
export const b = 2;
```

#### Do

```js
const a = 1;
const b = 2;
export { a, b };
```


### JSX

#### Don't

```jsx
export const a = 1;
export const b = 2;
```

#### Do

```jsx
const a = 1;
const b = 2;
export { a, b };
```


### TypeScript

#### Don't

```ts
export const a = 1;
export const b = 2;
```

#### Do

```ts
const a = 1;
const b = 2;
export { a, b };
```


### TSX

#### Don't

```tsx
export const a = 1;
export const b = 2;
```

#### Do

```tsx
const a = 1;
const b = 2;
export { a, b };
```
