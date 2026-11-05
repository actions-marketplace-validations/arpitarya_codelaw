---
id: js/max-classes-per-file
title: "One class per file"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the class moves to another file and its users import it"
fix_exception: restructure
stability: preview
ported_from: [eslint/max-classes-per-file]
since: 0.5.0
---

A file declares at most one class (the upstream default, `max: 1`). The second and later top-level classes are reported.

**Why:** A file per class keeps each one findable by its file name and its imports explicit.

```codelaw

match:
  any: [{kind: class_declaration}, {kind: class}]
  follows: { stopBy: end, any: [{kind: class_declaration}, {kind: export_statement, has: { kind: class_declaration }}, {kind: lexical_declaration, has: { stopBy: end, kind: class }}] }
  inside: { kind: program, stopBy: end }
message: "This file declares more than one class; move this one to its own file"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
class A {}
class B {}
```

#### Do

```js
class A {}
```

### JSX

#### Don't

```jsx
class A {}
class B {}
```

#### Do

```jsx
class A {}
```

### TypeScript

#### Don't

```ts
class A {}
class B {}
```

#### Do

```ts
class A {}
```

### TSX

#### Don't

```tsx
class A {}
class B {}
```

#### Do

```tsx
class A {}
```
