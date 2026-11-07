---
id: js/shorthand-property-grouping
title: "Shorthand properties grouped at the start or the end"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the order is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/shorthand-property-grouping]
since: 0.5.0
---

In an object literal, shorthand properties (`{ a }`) are all at the start or all at the end, not between `key: value` pairs.

**Why:** Grouped, a reader sees at once which values come from variables of the same name.

```codelaw
match:
  kind: shorthand_property_identifier
  inside: { kind: object }
  follows: { stopBy: end, kind: pair }
  precedes: { stopBy: end, kind: pair }
message: "Group shorthand properties at the start or the end"
```

## How to fix

1. Move the shorthand properties together, before or after the `key: value` pairs.

## Examples

### JavaScript

#### Don't

```js
const user = { id: 1, name, role: "admin", email };
```

#### Do

```js
const user = { name, email, id: 1, role: "admin" };
```


### JSX

#### Don't

```jsx
const user = { id: 1, name, role: "admin", email };
```

#### Do

```jsx
const user = { name, email, id: 1, role: "admin" };
```


### TypeScript

#### Don't

```ts
const user = { id: 1, name, role: "admin", email };
```

#### Do

```ts
const user = { name, email, id: 1, role: "admin" };
```


### TSX

#### Don't

```tsx
const user = { id: 1, name, role: "admin", email };
```

#### Do

```tsx
const user = { name, email, id: 1, role: "admin" };
```
