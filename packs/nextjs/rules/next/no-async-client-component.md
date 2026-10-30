---
id: next/no-async-client-component
title: "Client components are not async"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "the data loading moves to a server component or an effect, which restructures the code"
stability: preview
ported_from: ["@next/eslint-plugin-next/no-async-client-component"]
since: 0.5.0
---

In a file marked `"use client"`, a component (a capitalised function) is not `async`.

**Why:** React does not support async client components; they fail at run time or render nothing.

```codelaw
utils:
  client_file:
    kind: program
    has: {kind: expression_statement, nthChild: 1, regex: '^["'']use client["'']'}
match:
  any:
  - kind: function_declaration
    regex: ^async\b
    has: {field: name, regex: '^[A-Z]'}
  - kind: variable_declarator
    all:
    - has: {field: name, regex: '^[A-Z]'}
    - has: {field: value, kind: arrow_function, regex: ^async\b}
  inside: {stopBy: end, matches: client_file}
message: A client component cannot be `async`
```

## How to fix

1. Load the data in a server component and pass it as props, or use an effect or a data library in the client.

## Examples

### JavaScript

#### Don't

```js
'use client';
export default async function Page() {
  const d = await load();
  return <p>{d}</p>;
}
```

#### Do

```js
'use client';
export default function Page({ d }) {
  return <p>{d}</p>;
}
```

### JSX

#### Don't

```jsx
'use client';
export default async function Page() {
  const d = await load();
  return <p>{d}</p>;
}
```

#### Do

```jsx
'use client';
export default function Page({ d }) {
  return <p>{d}</p>;
}
```

### TSX

#### Don't

```tsx
'use client';
export default async function Page() {
  const d = await load();
  return <p>{d}</p>;
}
```

#### Do

```tsx
'use client';
export default function Page({ d }) {
  return <p>{d}</p>;
}
```
