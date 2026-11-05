---
id: js/object-shorthand
title: "{ x } rather than { x: x }"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint/object-shorthand]
since: 0.5.0
---

An object property whose value is a variable of the same name uses the shorthand (`{ x }`). Upstream also shortens methods; this port covers properties.

**Why:** The shorthand says the same with half the text.

```codelaw
match:
  kind: pair
  has:
    field: key
    kind: property_identifier
    pattern: $K
  all:
  - has:
      field: value
      kind: identifier
      pattern: $K
message: Write the shorthand `$K`
fix: $K
```

## Examples

### JavaScript

#### Don't

```js
const point = { x: x, y: y };
```

#### Do

```js
const point = { x, y };
```

### JSX

#### Don't

```jsx
const point = { x: x, y: y };
```

#### Do

```jsx
const point = { x, y };
```

### TypeScript

#### Don't

```ts
const point = { x: x, y: y };
```

#### Do

```ts
const point = { x, y };
```

### TSX

#### Don't

```tsx
const point = { x: x, y: y };
```

#### Do

```tsx
const point = { x, y };
```
