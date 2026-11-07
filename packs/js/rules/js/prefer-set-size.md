---
id: js/prefer-set-size
title: "Set size rather than spreading to an array"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-set-size]
since: 0.5.0
---

A new `Set`'s element count is read with `.size`, not by spreading it into an array (`[...new Set(x)].length`) or copying it with `Array.from`.

**Why:** `.size` is the count already kept; building an array only to measure it allocates for nothing.

```codelaw
match:
  any:
    - pattern: "[...new Set($X)].length"
    - pattern: "Array.from(new Set($X)).length"
message: "Read the set's `.size`"
fix: "new Set($X).size"
```

## Examples

### JavaScript

#### Don't

```js
const count = [...new Set(names)].length;
```

#### Do

```js
const count = new Set(names).size;
```


### JSX

#### Don't

```jsx
const count = [...new Set(names)].length;
```

#### Do

```jsx
const count = new Set(names).size;
```


### TypeScript

#### Don't

```ts
const count = [...new Set(names)].length;
```

#### Do

```ts
const count = new Set(names).size;
```


### TSX

#### Don't

```tsx
const count = [...new Set(names)].length;
```

#### Do

```tsx
const count = new Set(names).size;
```
