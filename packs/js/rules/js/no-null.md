---
id: js/no-null
title: "undefined rather than null"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "code that tests `=== null` elsewhere stops matching this value"
stability: preview
ported_from: [eslint-plugin-unicorn/no-null]
since: 0.5.0
---

`null` is not used as a value; `undefined` is. Comparisons with `null`, `Object.create(null)` and the like, and TypeScript types are left alone.

**Why:** Two empty values make every check ask which one it is; `undefined` is the one the language produces on its own.

```codelaw
match:
  kind: 'null'
  not:
    inside:
      kind: binary_expression
      has:
        field: operator
        regex: ^(===|!==|==|!=)$
  all:
  - not:
      inside:
        kind: arguments
        inside:
          kind: call_expression
          has:
            field: function
            regex: ^(Object\.create|Object\.setPrototypeOf|Reflect\.setPrototypeOf|JSON\.stringify)$
  - not:
      inside:
        kind: type_annotation
        stopBy: end
  - not:
      inside:
        kind: literal_type
message: Use `undefined` rather than `null`
fix: undefined
```

## Examples

### JavaScript

#### Don't

```js
let current = null;
```

#### Do

```js
let current = undefined;
```

### JSX

#### Don't

```jsx
let current = null;
```

#### Do

```jsx
let current = undefined;
```

### TypeScript

#### Don't

```ts
let current = null;
```

#### Do

```ts
let current = undefined;
```

### TSX

#### Don't

```tsx
let current = null;
```

#### Do

```tsx
let current = undefined;
```
