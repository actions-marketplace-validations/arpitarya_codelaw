---
id: js/no-shouty-constants
title: "No constant whose value is its own name"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the string is written where the constant was used and the declaration is removed; a constant kept as a single place to change the string stays"
stability: preview
ported_from: [biome/style/noShoutyConstants]
since: 0.5.1
---

A `const FOO = "FOO";` that is not exported and is read exactly once is reported. A `let`, an exported constant, one read more than once and a declaration of several names are left alone. The suggestion removes the declaration and writes the string at the use. Behaviour of Biome's `noShoutyConstants`.

**Why:** The name already says the value; the indirection adds a line and a lookup for the reader and nothing else.

```codelaw
native: js/no-shouty-constants
```

## Examples

### JavaScript

#### Don't

```js
const FOO = "FOO";
console.log(FOO);
```

#### Do

```js
console.log("FOO");
```

### JSX

#### Don't

```jsx
const FOO = "FOO";
console.log(FOO);
```

#### Do

```jsx
console.log("FOO");
```

### TypeScript

#### Don't

```ts
const FOO = "FOO";
console.log(FOO);
```

#### Do

```ts
console.log("FOO");
```

### TSX

#### Don't

```tsx
const FOO = "FOO";
console.log(FOO);
```

#### Do

```tsx
console.log("FOO");
```
