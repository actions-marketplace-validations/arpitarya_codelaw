---
id: js/no-map-spread
title: "No object spread of each item in map()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Object.assign` changes each item in place, so code still holding the original items sees the new properties"
stability: preview
ported_from: [oxlint/no-map-spread]
since: 0.5.0
---

A `map()` or `flatMap()` callback that adds properties to its item does so without copying the whole item with `{ ...item }`, when the original items are not used again. Only an arrow returning an object literal that starts with a spread of its parameter is reported.

**Why:** Spreading copies every property of every item, allocating a whole new object per element: on large arrays it is a measurable cost.

```codelaw
match:
  kind: parenthesized_expression
  pattern: "({ ...$P, $$$R })"
  has: { kind: object, has: { any: [{kind: pair}, {kind: shorthand_property_identifier}, {kind: method_definition}] } }
  inside:
    kind: arrow_function
    field: body
    any:
      - has: { field: parameter, pattern: $P }
      - has: { field: parameters, has: { nthChild: 1, pattern: $P } }
    inside:
      kind: arguments
      inside:
        kind: call_expression
        has: { field: function, kind: member_expression, has: { field: property, regex: ^(map|flatMap)$ } }
message: "This copies every `$P`; if the originals aren't reused, `Object.assign($P, {...})` adds the properties in place"
fix: "Object.assign($P, { $$$R })"
```

## Examples

### JavaScript

#### Don't

```js
const rows = users.map((u) => ({ ...u, label: u.name }));
```

#### Do

```js
const rows = users.map((u) => Object.assign(u, { label: u.name }));
```

### JSX

#### Don't

```jsx
const rows = users.map((u) => ({ ...u, label: u.name }));
```

#### Do

```jsx
const rows = users.map((u) => Object.assign(u, { label: u.name }));
```

### TypeScript

#### Don't

```ts
const rows = users.map((u) => ({ ...u, label: u.name }));
```

#### Do

```ts
const rows = users.map((u) => Object.assign(u, { label: u.name }));
```

### TSX

#### Don't

```tsx
const rows = users.map((u) => ({ ...u, label: u.name }));
```

#### Do

```tsx
const rows = users.map((u) => Object.assign(u, { label: u.name }));
```
