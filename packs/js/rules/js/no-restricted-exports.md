---
id: js/no-restricted-exports
title: "No export named default"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the statement is rewritten as `export default`; the reviewer confirms (a clause with several specifiers or a `from` source carries no edit)"
stability: preview
ported_from: [eslint/no-restricted-exports]
since: 0.5.0
---

A named export is not called `default` (`export { x as default }`). The upstream rule takes a list of restricted names; this port checks the one every project shares.

**Why:** `export { x as default }` is a default export written as a named one, which hides it from readers looking for `export default`.

```codelaw

match:
  any:
    - kind: export_specifier
      has: { field: alias, regex: '^default$' }
      all:
        - has: { field: name, kind: identifier, pattern: $N }
        - not: { precedes: { kind: export_specifier } }
        - not: { follows: { kind: export_specifier } }
        - inside:
            kind: export_clause
            stopBy: neighbor
            inside:
              kind: export_statement
              stopBy: neighbor
              pattern: $S
              not: { has: { field: source, pattern: $SRC } }
    - kind: export_specifier
      has: { field: alias, regex: '^default$' }
message: "Export it with `export default` rather than as `default`"
fix:
  - replace: { target: $S, with: "export default $N;" }
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
export { main as default };
```

#### Do

```js
export default main;
```

### JSX

#### Don't

```jsx
export { main as default };
```

#### Do

```jsx
export default main;
```

### TypeScript

#### Don't

```ts
export { main as default };
```

#### Do

```ts
export default main;
```

### TSX

#### Don't

```tsx
export { main as default };
```

#### Do

```tsx
export default main;
```
