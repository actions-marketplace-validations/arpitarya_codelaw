---
id: js/consistent-boolean-name
title: "A variable holding a boolean is named with is, has, can, should …"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the new name is the old one with `is` in front, which may not be the best prefix, and may collide with a name declared elsewhere; the rename is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-boolean-name]
since: 0.5.0
---

A `const` or `let` whose value is plainly a boolean — `true`, `false`, a comparison, `!x`, `Boolean(x)`, `instanceof` or `in` — has a name that starts with a boolean prefix (`is`, `are`, `has`, `have`, `can`, `should`, `was`, `were`, `does`, `did`, `will`, `requires`) as its own word: `isCompleted`, not `completed`. Upstream also checks parameters, functions, type annotations and misleading prefixes on non-booleans; those are not reported here. Exported names are left: renaming them changes the module's interface.

**Why:** The prefix tells the reader the name holds a yes-or-no without looking at its value.

```codelaw
match:
  kind: identifier
  inside:
    kind: variable_declarator
    field: name
    inside: { kind: lexical_declaration, not: { inside: { kind: export_statement } } }
    has:
      field: value
      any:
        - kind: "true"
        - kind: "false"
        - kind: unary_expression
          regex: '^!'
        - kind: binary_expression
          has: { field: operator, regex: '^(===|!==|==|!=|<|>|<=|>=|instanceof|in)$' }
        - kind: call_expression
          regex: '^Boolean\('
  not:
    any:
      - regex: '^_*(is|are|has|have|can|should|was|were|does|did|will|requires)([A-Z0-9_]|$)'
      - regex: '^_*(IS|ARE|HAS|HAVE|CAN|SHOULD|WAS|WERE|DOES|DID|WILL|REQUIRES)_'
      - regex: '^[A-Z0-9_]+$'
  pattern: $N
transform:
  PASCAL: { convert: { source: $N, toCase: pascalCase } }
message: "Name the boolean `$N` with a prefix such as `is`"
fix:
  - rename-symbol-local: { target: $N, to: "is$PASCAL" }
```

## Examples

### JavaScript

#### Don't

```js
const completed = progress === 100;
show(completed);
```

#### Do

```js
const isCompleted = progress === 100;
show(isCompleted);
```


### JSX

#### Don't

```jsx
const completed = progress === 100;
show(completed);
```

#### Do

```jsx
const isCompleted = progress === 100;
show(isCompleted);
```


### TypeScript

#### Don't

```ts
const completed = progress === 100;
show(completed);
```

#### Do

```ts
const isCompleted = progress === 100;
show(isCompleted);
```


### TSX

#### Don't

```tsx
const completed = progress === 100;
show(completed);
```

#### Do

```tsx
const isCompleted = progress === 100;
show(isCompleted);
```
