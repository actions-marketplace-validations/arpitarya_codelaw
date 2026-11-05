---
id: next/no-typos
title: "Data-fetching functions are spelled right"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the intended name is almost always obvious, but renaming an export is confirmed by the author"
fix_pending: "suggest"
files: ["**/pages/**"]
stability: preview
ported_from: ["@next/eslint-plugin-next/no-typos"]
since: 0.5.0
---

An exported `getStaticProps`, `getStaticPaths` or `getServerSideProps` in a page is spelled exactly so.

**Why:** Next.js finds them by name; a misspelled one is never called and the page silently gets no data.

```codelaw
match:
  any:
    - kind: function_declaration
      has: { field: name, pattern: $N }
      inside: { kind: export_statement }
    - kind: variable_declarator
      has: { field: name, pattern: $N }
      inside: { kind: lexical_declaration, inside: { kind: export_statement } }
constraints:
  N:
    regex: '^(?i:getstaticprops?|getstaticpaths?|getserversideprops?|getstaticprop|getserversideprop|getstaticpath)$'
    not: { regex: '^(getStaticProps|getStaticPaths|getServerSideProps)$' }
message: "`$N` looks like a misspelled Next.js data function"
```

## How to fix

1. Rename it to `getStaticProps`, `getStaticPaths` or `getServerSideProps`.

## Examples

### JavaScript

#### Don't

```js path=pages/index.js
export async function getServersideProps() {
  return { props: {} };
}
```

#### Do

```js path=pages/index.js
export async function getServerSideProps() {
  return { props: {} };
}
```

### JSX

#### Don't

```jsx path=pages/index.jsx
export async function getServersideProps() {
  return { props: {} };
}
```

#### Do

```jsx path=pages/index.jsx
export async function getServerSideProps() {
  return { props: {} };
}
```

### TypeScript

#### Don't

```ts path=pages/index.ts
export async function getServersideProps() {
  return { props: {} };
}
```

#### Do

```ts path=pages/index.ts
export async function getServerSideProps() {
  return { props: {} };
}
```

### TSX

#### Don't

```tsx path=pages/index.tsx
export async function getServersideProps() {
  return { props: {} };
}
```

#### Do

```tsx path=pages/index.tsx
export async function getServerSideProps() {
  return { props: {} };
}
```

## Differences from the source

Names differing only in case or a missing final `s` are caught; the source also catches other one-letter typos.
