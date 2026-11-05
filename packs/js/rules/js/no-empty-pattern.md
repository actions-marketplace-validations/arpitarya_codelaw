---
id: js/no-empty-pattern
title: "No empty destructuring patterns"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "both edits change behaviour (the empty pattern throws on null today): the reviewer picks a default or the bare name; an empty pattern that is not a property's value has no edit"
stability: preview
ported_from: [eslint/no-empty-pattern]
since: 0.1.0
---

A destructuring pattern names at least one binding.

**Why:** `const {} = obj` binds nothing; it's usually a mistyped default such as `{ a = {} }`.

```codelaw
match:
  any:
    - kind: object_pattern
      regex: ^\{\s*\}$
      inside:
        kind: pair_pattern
        pattern: $P
        has: { field: key, pattern: $K }
    - kind: object_pattern
      regex: ^\{\s*\}$
    - kind: array_pattern
      regex: ^\[\s*\]$
message: "This destructuring pattern binds nothing"
fix:
  choices:
    - label: default to an empty object
      fix:
        - replace: { target: $P, with: "$K = {}" }
    - label: bind the value by name
      fix:
        - replace: { target: $P, with: "$K" }
```

## How to fix

1. Name the properties to bind inside the pattern, or remove it.

## Examples

### JavaScript

#### Don't

```js
const { a: {} } = options;
```

#### Fixed (default to an empty object)

```js
const { a = {} } = options;
```

#### Fixed (bind the value by name)

```js
const { a } = options;
```

#### Do

```js
const { a = {} } = options;
```

### JSX

#### Don't

```jsx
const { a: {} } = options;
```

#### Fixed (default to an empty object)

```jsx
const { a = {} } = options;
```

#### Fixed (bind the value by name)

```jsx
const { a } = options;
```

#### Do

```jsx
const { a = {} } = options;
```

### TypeScript

#### Don't

```ts
const { a: {} } = options;
```

#### Fixed (default to an empty object)

```ts
const { a = {} } = options;
```

#### Fixed (bind the value by name)

```ts
const { a } = options;
```

#### Do

```ts
const { a = {} } = options;
```

### TSX

#### Don't

```tsx
const { a: {} } = options;
```

#### Fixed (default to an empty object)

```tsx
const { a = {} } = options;
```

#### Fixed (bind the value by name)

```tsx
const { a } = options;
```

#### Do

```tsx
const { a = {} } = options;
```
