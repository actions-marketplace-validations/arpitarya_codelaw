---
id: js/no-useless-boolean-cast
title: "No Boolean() around a predicate callback's result"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-boolean-cast]
since: 0.5.0
---

`filter`, `some`, `every`, `find`, `findIndex`, `findLast` and `findLastIndex` read their callback's result as truthy or falsy, so a callback that returns `Boolean(x)` can return `x`. A cast around an optional chain is left: it turns `undefined` into `false` on purpose.

**Why:** The array method already does the conversion; the cast repeats it.

```codelaw
match:
  kind: call_expression
  pattern: "Boolean($E)"
  inside:
    kind: arrow_function
    field: body
    nthChild: 1
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has: { field: function, kind: member_expression, has: { field: property, regex: '^(filter|some|every|find|findIndex|findLast|findLastIndex)$' } }
  not: { has: { field: arguments, has: { stopBy: end, kind: optional_chain } } }
constraints:
  E: { not: { kind: spread_element } }
message: "The array method already reads truthiness; drop `Boolean`"
fix: "$E"
```

## Examples

### JavaScript

#### Don't

```js
const active = users.filter((user) => Boolean(user.active));
```

#### Do

```js
const active = users.filter((user) => user.active);
```


### JSX

#### Don't

```jsx
const active = users.filter((user) => Boolean(user.active));
```

#### Do

```jsx
const active = users.filter((user) => user.active);
```


### TypeScript

#### Don't

```ts
const active = users.filter((user) => Boolean(user.active));
```

#### Do

```ts
const active = users.filter((user) => user.active);
```


### TSX

#### Don't

```tsx
const active = users.filter((user) => Boolean(user.active));
```

#### Do

```tsx
const active = users.filter((user) => user.active);
```
