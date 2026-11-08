---
id: js/consistent-export-decorator-position
title: "Class decorators before export, not between export and class"
languages: [typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-export-decorator-position]
since: 0.5.0
---

A decorated exported class puts its decorators before `export` (`@decorator` on its own line, then `export default class Foo {}`), not between `export` and `class` (`export default @decorator class Foo {}`). Upstream can be set to require the other position; this rule checks the default one, and whether the decorator shares a line with `export` is the formatter's.

**Why:** Both positions are valid; one throughout the code base keeps exports easy to scan.

```codelaw
match:
  kind: decorator
  inside:
    kind: class_declaration
    field: decorator
    inside: { kind: export_statement, field: declaration, pattern: $EXPORT }
message: "Put the decorator before `export`"
fix:
  - move: { before: $EXPORT }
```

## Examples

### TypeScript

#### Don't

```ts
export default @sealed class Store {}
```

#### Do

```ts
@sealed
export default class Store {}
```


### TSX

#### Don't

```tsx
export default @sealed class Store {}
```

#### Do

```tsx
@sealed
export default class Store {}
```
