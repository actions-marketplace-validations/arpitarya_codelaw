---
id: js/no-optional-chaining-on-undeclared-variable
title: "No ?. on an undeclared name"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the name is a global or a typo is the author's call"
stability: preview
ported_from: [eslint-plugin-unicorn/no-optional-chaining-on-undeclared-variable]
since: 0.5.0
---

Optional chaining is not applied to a name that is declared nowhere in the file and is not a known global.

**Why:** `?.` guards against `null` and `undefined`, not against a missing variable, which throws a ReferenceError.

```codelaw

match:
  kind: member_expression
  regex: '^[A-Za-z_$][\w$]*\?\.'
  has: { field: object, kind: identifier, pattern: $X }
where:
  $X: undeclared
message: "`$X?.` still throws when `$X` is not declared; use `typeof $X !== \"undefined\"` or `globalThis.$X?.`"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const v = config?.version;
```

#### Do

```js
const v = globalThis.config?.version;
```

### JSX

#### Don't

```jsx
const v = config?.version;
```

#### Do

```jsx
const v = globalThis.config?.version;
```

### TypeScript

#### Don't

```ts
const v = config?.version;
```

#### Do

```ts
const v = globalThis.config?.version;
```

### TSX

#### Don't

```tsx
const v = config?.version;
```

#### Do

```tsx
const v = globalThis.config?.version;
```
