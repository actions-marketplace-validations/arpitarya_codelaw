---
id: js/no-useless-coercion
title: "No String(\"literal\")"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-coercion]
since: 0.5.0
---

`String`, `Number` and `Boolean` are not called on a literal of their own type.

**Why:** The conversion does nothing.

```codelaw

match:
  any:
    - { pattern: String($X), has: { field: arguments, has: { any: [{kind: string}, {kind: template_string}] } } }
    - { pattern: Number($X), has: { field: arguments, has: { kind: number } } }
    - { pattern: Boolean($X), has: { field: arguments, has: { any: [{kind: "true"}, {kind: "false"}] } } }
message: "`$X` already has that type"
fix: "$X"
```

## Examples

### JavaScript

#### Don't

```js
const label = String("total");
```

#### Do

```js
const label = "total";
```

### JSX

#### Don't

```jsx
const label = String("total");
```

#### Do

```jsx
const label = "total";
```

### TypeScript

#### Don't

```ts
const label = String("total");
```

#### Do

```ts
const label = "total";
```

### TSX

#### Don't

```tsx
const label = String("total");
```

#### Do

```tsx
const label = "total";
```
