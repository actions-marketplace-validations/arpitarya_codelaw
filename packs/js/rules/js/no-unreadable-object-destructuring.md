---
id: js/no-unreadable-object-destructuring
title: "No computed key or nested array in an object destructuring"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "rewriting the destructuring into separate reads needs the author's choice of statements"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-unreadable-object-destructuring]
since: 0.5.0
---

`const { [key]: value } = object` is `const value = object[key]`, and `const { foo: [bar] } = object` is `const [bar] = object.foo`: the plain forms read in the order they run. A computed key next to a rest element (`{ [key]: value, ...rest }`) is the only way to leave that key out of the rest, and stays.

**Why:** A computed key or an array pattern inside an object pattern packs two operations into one, and the reader has to unpack them.

```codelaw
match:
  kind: pair_pattern
  inside: { kind: object_pattern, not: { has: { kind: rest_pattern } } }
  any:
    - has: { field: key, kind: computed_property_name }
    - has: { field: value, kind: array_pattern }
message: "Read this property in its own statement"
```

## How to fix

1. For a computed key, write `const value = object[key];`.
2. For a nested array, write `const [bar] = object.foo;`.

## Examples

### JavaScript

#### Don't

```js
const { [key]: value } = settings;
```

#### Do

```js
const value = settings[key];
```


### JSX

#### Don't

```jsx
const { [key]: value } = settings;
```

#### Do

```jsx
const value = settings[key];
```


### TypeScript

#### Don't

```ts
const { [key]: value } = settings;
```

#### Do

```ts
const value = settings[key];
```


### TSX

#### Don't

```tsx
const { [key]: value } = settings;
```

#### Do

```tsx
const value = settings[key];
```
