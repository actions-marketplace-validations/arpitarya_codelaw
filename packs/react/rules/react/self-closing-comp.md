---
id: react/self-closing-comp
title: "An element with no children closes itself"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react/self-closing-comp]
since: 0.5.0
---

An element with no children is written self-closing: `<Spinner />`, not `<Spinner></Spinner>`. For now only an element with nothing at all between its tags is reported.

**Why:** The closing tag carries no information when there is nothing between the tags.

```codelaw
match:
  kind: jsx_element
  all:
    - has: { field: open_tag, not: { regex: '^<>$' }, has: { nthChild: { position: 1, reverse: true }, pattern: $L } }
    - has: { field: close_tag, pattern: $C }
    - not: { has: { any: [{ kind: jsx_element }, { kind: jsx_self_closing_element }, { kind: jsx_expression }, { kind: jsx_text }] } }
message: "Write this element self-closing"
fix:
  - insert-after: { target: $L, text: " /" }
  - remove: $C
```

## Examples

### JavaScript

#### Don't

```js
const s = <Spinner size="sm"></Spinner>;
```

#### Do

```js
const s = <Spinner size="sm" />;
```

### JSX

#### Don't

```jsx
const s = <Spinner size="sm"></Spinner>;
```

#### Do

```jsx
const s = <Spinner size="sm" />;
```

### TSX

#### Don't

```tsx
const s = <Spinner size="sm"></Spinner>;
```

#### Do

```tsx
const s = <Spinner size="sm" />;
```
