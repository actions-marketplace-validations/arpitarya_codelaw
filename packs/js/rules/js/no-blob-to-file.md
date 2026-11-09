---
id: js/no-blob-to-file
title: "No `File` made from a `Blob` only to be uploaded or turned into a URL"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the uploaded part's name moves to the `append()` call, which the author should confirm"
stability: preview
ported_from: [eslint-plugin-unicorn/no-blob-to-file]
since: 0.5.0
---

`const f = new File([blob], "name")`, where `blob` is a `new Blob()` (or a `File`) and `f`'s only use is `URL.createObjectURL(f)` or `formData.append(key, f)`/`set(key, f)` on a fresh `new FormData()`, is reported at `new File(…)`: both take the `Blob` directly (`append` takes the name as its third argument). The suggestion removes `f`, passes the `Blob` and moves the name into `append()`.

**Why:** The `File` copies nothing but adds an object and a name the call can take itself.

```codelaw
native: js/no-blob-to-file
```

## Examples

### JavaScript

#### Don't

```js
const blob = new Blob([data]);
const file = new File([blob], "a.txt");
const url = URL.createObjectURL(file);
```

#### Do

```js
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```

#### Fixed

```js
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```


### JSX

#### Don't

```jsx
const blob = new Blob([data]);
const file = new File([blob], "a.txt");
const url = URL.createObjectURL(file);
```

#### Do

```jsx
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```

#### Fixed

```jsx
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```


### TypeScript

#### Don't

```ts
const blob = new Blob([data]);
const file = new File([blob], "a.txt");
const url = URL.createObjectURL(file);
```

#### Do

```ts
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```

#### Fixed

```ts
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```


### TSX

#### Don't

```tsx
const blob = new Blob([data]);
const file = new File([blob], "a.txt");
const url = URL.createObjectURL(file);
```

#### Do

```tsx
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```

#### Fixed

```tsx
const blob = new Blob([data]);
const url = URL.createObjectURL(blob);
```
