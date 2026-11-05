---
id: js/dot-notation
title: "obj.key rather than obj[\"key\"]"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/dot-notation]
since: 0.5.0
---

A property whose name is a valid identifier is read with dot notation.

**Why:** `obj.key` is shorter and lets editors and tools follow the property.

```codelaw
match:
  kind: subscript_expression
  has:
    field: object
    pattern: $O
  not:
    regex: \?\.
  all:
  - has:
      field: index
      kind: string
      has:
        kind: string_fragment
        regex: ^[A-Za-z_$][A-Za-z0-9_$]*$
        pattern: $K
message: Write `$O.$K`
fix: $O.$K
```

## Examples

### JavaScript

#### Don't

```js
const id = user["id"];
```

#### Do

```js
const id = user.id;
```

### JSX

#### Don't

```jsx
const id = user["id"];
```

#### Do

```jsx
const id = user.id;
```

### TypeScript

#### Don't

```ts
const id = user["id"];
```

#### Do

```ts
const id = user.id;
```

### TSX

#### Don't

```tsx
const id = user["id"];
```

#### Do

```tsx
const id = user.id;
```
