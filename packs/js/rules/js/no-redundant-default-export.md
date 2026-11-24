---
id: js/no-redundant-default-export
title: "No default export of a name that is also exported by name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which of the two forms importers use is a decision across files"
fix_exception: restructure
stability: preview
ported_from: [biome/complexity/noRedundantDefaultExport]
since: 0.5.1
---

`export default foo;` is reported when the same file exports `foo` by name (`export const foo`, `export function foo`, `export { foo }`). Re-exports (`export … from`) are not read. Behaviour of Biome's `noRedundantDefaultExport`.

**Why:** Two ways to import one binding invite two spellings across the project, and tools that rename the export have to find both.

```codelaw
native: js/no-redundant-default-export
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
export const foo = 42;
export default foo;
```

#### Do

```js
export const foo = 42;
```

### JSX

#### Don't

```jsx
export const foo = 42;
export default foo;
```

#### Do

```jsx
export const foo = 42;
```

### TypeScript

#### Don't

```ts
export const foo = 42;
export default foo;
```

#### Do

```ts
export const foo = 42;
```

### TSX

#### Don't

```tsx
export const foo = 42;
export default foo;
```

#### Do

```tsx
export const foo = 42;
```
