---
id: js/prefer-single-object-destructuring
title: "One destructuring of an object, not two in a row"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-single-object-destructuring]
since: 0.5.0
---

Two `const` destructurings of the same variable in a row are one: `const { a } = o; const { b } = o;` is `const { a, b } = o;`.

**Why:** One statement shows everything taken from the object.

```codelaw
match:
  kind: lexical_declaration
  pattern: "const { $$$A } = $O;"
  precedes: { pattern: "const { $$$B } = $O;", all: [{ pattern: $SECOND }] }
  all: [{ pattern: $FIRST }]
constraints:
  O: { kind: identifier }
message: "Destructure `$O` once"
fix:
  - replace: { target: $FIRST, with: "const { $$$A, $$$B } = $O;" }
  - remove: $SECOND
```

## Examples

### JavaScript

#### Don't

```js
const { name } = props;
const { age } = props;
```

#### Do

```js
const { name, age } = props;
```


### JSX

#### Don't

```jsx
const { name } = props;
const { age } = props;
```

#### Do

```jsx
const { name, age } = props;
```


### TypeScript

#### Don't

```ts
const { name } = props;
const { age } = props;
```

#### Do

```ts
const { name, age } = props;
```


### TSX

#### Don't

```tsx
const { name } = props;
const { age } = props;
```

#### Do

```tsx
const { name, age } = props;
```
