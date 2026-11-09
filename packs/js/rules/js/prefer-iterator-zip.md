---
id: js/prefer-iterator-zip
title: "`Iterator.zip()` for a loop that indexes parallel arrays"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the new loop reads every row before the body and initialises each iterator up front, and `Iterator.zip` is not in every runtime yet; the author accepts that"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-iterator-zip]
since: 0.5.0
---

A `for` loop with a block body, a `let` index starting at `0`, stepped by one (`i++`, `++i`, `i += 1`, `i = i + 1`) and bounded by the shortest of two or more arrays — `i < Math.min(a.length, b.length)` or `i < a.length && i < b.length`, either way round, the `Math.min` result also cached in the loop head or in a `const`/`let` just before the loop that nothing else reads — whose body only reads `a[i]`, `b[i]` for those distinct identifiers, is reported on its head. The suggestion is `for (const [x, y] of Iterator.zip([a, b]))` with each read replaced by its name (the singular of the array's name, made unique). Single-input bounds, callbacks that capture the index or an array, property chains, optional indexing, offsets, writes, any other use of the index or of an array, and a string or plain object bound are left alone; a comment inside what the fix would rewrite leaves the finding without an edit. Behaviour of eslint-plugin-unicorn v79.0.0 (read, not copied).

**Why:** Reading several arrays at one index says "walk them together" in a way the reader has to reconstruct; `Iterator.zip` says it and stops at the shortest by itself. No engine ships it yet, so check the runtime before applying the suggestion.

```codelaw
native: js/prefer-iterator-zip
```

## Examples

### JavaScript

#### Don't

```js
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
  save(names[index], scores[index]);
}
```

#### Do

```js
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```

#### Fixed

```js
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```


### JSX

#### Don't

```jsx
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
  save(names[index], scores[index]);
}
```

#### Do

```jsx
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```

#### Fixed

```jsx
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```


### TypeScript

#### Don't

```ts
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
  save(names[index], scores[index]);
}
```

#### Do

```ts
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```

#### Fixed

```ts
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```


### TSX

#### Don't

```tsx
for (let index = 0; index < Math.min(names.length, scores.length); index++) {
  save(names[index], scores[index]);
}
```

#### Do

```tsx
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```

#### Fixed

```tsx
for (const [name, score] of Iterator.zip([names, scores])) {
  save(name, score);
}
```
