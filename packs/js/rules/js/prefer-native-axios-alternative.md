---
id: js/prefer-native-axios-alternative
title: "Promise.all rather than axios.all"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-native-axios-alternative]
since: 0.5.0
---

`axios.all(requests)` is a deprecated alias of `Promise.all(requests)`; the standard call is used. Upstream also reports `axios.spread` (destructure the results instead) and `CancelToken` (use an `AbortController` signal); those need the callback or the cancellation rewritten, and are not reported here.

**Why:** The alias is deprecated and adds nothing over the built-in.

```codelaw
match:
  pattern: "axios.all($X)"
message: "Use `Promise.all`"
fix: "Promise.all($X)"
```

## Examples

### JavaScript

#### Don't

```js
const [a, b] = await axios.all([axios.get("/a"), axios.get("/b")]);
```

#### Do

```js
const [a, b] = await Promise.all([axios.get("/a"), axios.get("/b")]);
```


### JSX

#### Don't

```jsx
const [a, b] = await axios.all([axios.get("/a"), axios.get("/b")]);
```

#### Do

```jsx
const [a, b] = await Promise.all([axios.get("/a"), axios.get("/b")]);
```


### TypeScript

#### Don't

```ts
const [a, b] = await axios.all([axios.get("/a"), axios.get("/b")]);
```

#### Do

```ts
const [a, b] = await Promise.all([axios.get("/a"), axios.get("/b")]);
```


### TSX

#### Don't

```tsx
const [a, b] = await axios.all([axios.get("/a"), axios.get("/b")]);
```

#### Do

```tsx
const [a, b] = await Promise.all([axios.get("/a"), axios.get("/b")]);
```
