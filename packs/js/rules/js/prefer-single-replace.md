---
id: js/prefer-single-replace
title: "One `replaceAll()` with a character class for a chain of one-character replacements"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-single-replace]
since: 0.5.0
---

A chain of two or more `.replaceAll("c", r)` or `.replace(/c/g, r)` calls that each replace one character with the same string `r` is reported at the outermost call and rewritten as one `.replaceAll(/[…]/g, r)`. A replacement that holds one of the replaced characters, `` $` `` or `$'`, a search that is not one character, and a regular expression with flags other than `g`, `m` and `s` are left alone.

**Why:** One pass over the string with one pattern does what the chain does in several.

```codelaw
native: js/prefer-single-replace
```

## Examples

### JavaScript

#### Don't

```js
const slug = title.replaceAll(" ", "-").replace(/_/g, "-");
```

#### Do

```js
const slug = title.replaceAll(/[ _]/g, "-");
```

#### Fixed

```js
const slug = title.replaceAll(/[ _]/g, "-");
```


### JSX

#### Don't

```jsx
const slug = title.replaceAll(" ", "-").replace(/_/g, "-");
```

#### Do

```jsx
const slug = title.replaceAll(/[ _]/g, "-");
```

#### Fixed

```jsx
const slug = title.replaceAll(/[ _]/g, "-");
```


### TypeScript

#### Don't

```ts
const slug = title.replaceAll(" ", "-").replace(/_/g, "-");
```

#### Do

```ts
const slug = title.replaceAll(/[ _]/g, "-");
```

#### Fixed

```ts
const slug = title.replaceAll(/[ _]/g, "-");
```


### TSX

#### Don't

```tsx
const slug = title.replaceAll(" ", "-").replace(/_/g, "-");
```

#### Do

```tsx
const slug = title.replaceAll(/[ _]/g, "-");
```

#### Fixed

```tsx
const slug = title.replaceAll(/[ _]/g, "-");
```
