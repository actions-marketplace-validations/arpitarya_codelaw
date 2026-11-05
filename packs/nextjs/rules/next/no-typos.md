---
id: next/no-typos
title: "Data-fetching functions are spelled right"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the name is mapped to the one data function it differs from only in case or a final `s`; the reviewer confirms the rename of the export"
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
      all:
        - has: { field: name, pattern: $N }
        - has: { field: name, pattern: $P, regex: '^(?i:getstaticprops?)$' }
      inside: { kind: export_statement }
    - kind: variable_declarator
      all:
        - has: { field: name, pattern: $N }
        - has: { field: name, pattern: $P, regex: '^(?i:getstaticprops?)$' }
      inside: { kind: lexical_declaration, inside: { kind: export_statement } }
    - kind: function_declaration
      all:
        - has: { field: name, pattern: $N }
        - has: { field: name, pattern: $T, regex: '^(?i:getstaticpaths?)$' }
      inside: { kind: export_statement }
    - kind: variable_declarator
      all:
        - has: { field: name, pattern: $N }
        - has: { field: name, pattern: $T, regex: '^(?i:getstaticpaths?)$' }
      inside: { kind: lexical_declaration, inside: { kind: export_statement } }
    - kind: function_declaration
      all:
        - has: { field: name, pattern: $N }
        - has: { field: name, pattern: $S, regex: '^(?i:getserversideprops?)$' }
      inside: { kind: export_statement }
    - kind: variable_declarator
      all:
        - has: { field: name, pattern: $N }
        - has: { field: name, pattern: $S, regex: '^(?i:getserversideprops?)$' }
      inside: { kind: lexical_declaration, inside: { kind: export_statement } }
constraints:
  N:
    regex: '^(?i:getstaticprops?|getstaticpaths?|getserversideprops?|getstaticprop|getserversideprop|getstaticpath)$'
    not: { regex: '^(getStaticProps|getStaticPaths|getServerSideProps)$' }
message: "`$N` looks like a misspelled Next.js data function"
fix:
  - replace: { target: $P, with: "getStaticProps" }
  - replace: { target: $T, with: "getStaticPaths" }
  - replace: { target: $S, with: "getServerSideProps" }
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
