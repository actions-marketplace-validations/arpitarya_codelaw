---
id: js/prefer-location-assign
title: "location.assign rather than location = url"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-location-assign]
since: 0.5.0
---

Navigation is written `location.assign(url)`, not an assignment to `location`.

**Why:** Assigning to the global object `location` looks like a variable write.

```codelaw
match:
  pattern: "location = $X"
message: "Use `location.assign()`"
fix: "location.assign($X)"
```

## Examples

### JavaScript

#### Don't

```js
location = "/login";
```

#### Do

```js
location.assign("/login");
```

### JSX

#### Don't

```jsx
location = "/login";
```

#### Do

```jsx
location.assign("/login");
```

### TypeScript

#### Don't

```ts
location = "/login";
```

#### Do

```ts
location.assign("/login");
```

### TSX

#### Don't

```tsx
location = "/login";
```

#### Do

```tsx
location.assign("/login");
```
