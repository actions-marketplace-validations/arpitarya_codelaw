---
id: js/no-unnecessary-callback-wrapper
title: "No arrow that only forwards to a function"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "passing `f` directly also passes the caller's extra arguments (`map` passes the index too) and drops the wrapper's `this`"
stability: preview
ported_from: [tslint/no-unnecessary-callback-wrapper]
since: 0.5.0
---

An arrow function that only passes its parameters, in order, to a named function (`(x) => f(x)`) is replaced by that function (`f`).

**Why:** The wrapper adds a call and a name to read and does nothing.

```codelaw
match:
  kind: arrow_function
  not: { regex: '^async\b' }
  all:
    - has: { field: parameters, kind: formal_parameters, all: [{ has: { nthChild: 1, any: [{ kind: identifier, pattern: $A }, { kind: required_parameter, has: { field: pattern, kind: identifier, pattern: $A } }] } }, { not: { has: { nthChild: 2 } } }] }
    - has: { field: body, kind: call_expression, all: [{ has: { field: function, kind: identifier, pattern: $F } }, { has: { field: arguments, all: [{ has: { nthChild: 1, pattern: $A } }, { not: { has: { nthChild: 2 } } }] } }] }
message: "`$$MATCH` only forwards to `$F`; pass `$F`"
fix: "$F"
```

## Examples

### JavaScript

#### Don't

```js
const names = users.map((u) => format(u));
```

#### Do

```js
const names = users.map(format);
```

### JSX

#### Don't

```jsx
const names = users.map((u) => format(u));
```

#### Do

```jsx
const names = users.map(format);
```

### TypeScript

#### Don't

```ts
const names = users.map((u) => format(u));
```

#### Do

```ts
const names = users.map(format);
```

### TSX

#### Don't

```tsx
const names = users.map((u) => format(u));
```

#### Do

```tsx
const names = users.map(format);
```
