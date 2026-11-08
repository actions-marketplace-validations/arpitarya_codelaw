---
id: js/destructuring-assignment-syntax
title: "Destructure two properties read from the same object in a row"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the two declarations become one; a comment between them, or a getter with side effects in a different order, needs review"
stability: preview
ported_from: [eslint-plugin-sonarjs/destructuring-assignment-syntax]
since: 0.5.0
---

`const a = obj.a; const b = obj.b;` — two consecutive declarations of the same kind, each naming a variable after the property it reads from one object — is `const { a, b } = obj;`. Upstream also groups three or more and array indexes; this rule handles the pair.

**Why:** One destructuring names the source once and the properties together.

```codelaw
match:
  kind: lexical_declaration
  any:
    - pattern: "const $A = $O.$A;"
    - pattern: "let $A = $O.$A;"
  precedes:
    all:
      - any:
          - pattern: "const $B = $O.$B;"
          - pattern: "let $B = $O.$B;"
      - pattern: $NEXT
  has: { kind: variable_declarator, pattern: $DECL }
constraints:
  O: { any: [{ kind: identifier }, { kind: this }] }
message: "Destructure `$A` and `$B` from `$O` together"
fix:
  - replace: { target: $DECL, with: "{ $A, $B } = $O" }
  - remove: $NEXT
```

## Examples

### JavaScript

#### Don't

```js
const width = size.width;
const height = size.height;
```

#### Do

```js
const { width, height } = size;
```


### JSX

#### Don't

```jsx
const width = size.width;
const height = size.height;
```

#### Do

```jsx
const { width, height } = size;
```


### TypeScript

#### Don't

```ts
const width = size.width;
const height = size.height;
```

#### Do

```ts
const { width, height } = size;
```


### TSX

#### Don't

```tsx
const width = size.width;
const height = size.height;
```

#### Do

```tsx
const { width, height } = size;
```
