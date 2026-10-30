---
id: react/no-array-index-key
title: "No array index as a key"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: a stable key comes from the item's data (an id), which only the author knows"
stability: preview
ported_from: [eslint-plugin-react/no-array-index-key]
since: 0.5.0
---

A `key` is not the index a `map`, `forEach`, `filter`, `some`, `every`, `find`, `findIndex`, `flatMap` or `reduce` callback receives — alone, in a template literal, or through `String(i)` or `i.toString()`.

**Why:** An index key moves with position, so inserting or reordering items makes React reuse the wrong element and its state.

```codelaw
match:
  kind: jsx_attribute
  regex: '^key\s*='
  all:
    - inside:
        stopBy: end
        any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, kind: member_expression, regex: '\.(map|forEach|filter|some|every|find|findIndex|flatMap)$' } } }
        has: { field: parameters, has: { nthChild: 2, any: [{ kind: identifier, pattern: $I }, { kind: required_parameter, has: { field: pattern, kind: identifier, pattern: $I } }] } }
    - has:
        kind: jsx_expression
        has:
          any:
            - { kind: identifier, pattern: $I }
            - { kind: template_string, has: { kind: template_substitution, has: { kind: identifier, pattern: $I } } }
            - { kind: call_expression, pattern: String($I) }
            - { kind: call_expression, pattern: $I.toString() }
message: "`$I` is the array index; key by the item's id"
```

## How to fix

1. Use a stable id from the item as the key: `key={item.id}`.

## Examples

### JavaScript

#### Don't

```js
const l = items.map((item, i) => <li key={i}>{item.name}</li>);
```

#### Do

```js
const l = items.map((item) => <li key={item.id}>{item.name}</li>);
```

### JSX

#### Don't

```jsx
const l = items.map((item, i) => <li key={i}>{item.name}</li>);
```

#### Do

```jsx
const l = items.map((item) => <li key={item.id}>{item.name}</li>);
```

### TSX

#### Don't

```tsx
const l = items.map((item, i) => <li key={i}>{item.name}</li>);
```

#### Do

```tsx
const l = items.map((item) => <li key={item.id}>{item.name}</li>);
```
