---
id: js/prefer-url-can-parse
title: "URL.canParse() rather than try-catch around new URL()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-url-can-parse]
since: 0.5.0
---

Whether a string is a valid URL is tested with `URL.canParse(input)`, not by constructing a `URL` in a `try` that returns `true` and a `catch` that returns `false`.

**Why:** `URL.canParse` answers the question without building an object or throwing.

```codelaw
match:
  any:
    - pattern: "try { new URL($$$A); return true; } catch { return false; }"
      not: { has: { kind: finally_clause } }
    - pattern: "try { new URL($$$A); return true; } catch ($E) { return false; }"
      not: { has: { kind: finally_clause } }
message: "Return `URL.canParse(...)`"
fix: "return URL.canParse($$$A);"
```

## Examples

### JavaScript

#### Don't

```js
function isUrl(text) {
  try {
    new URL(text);
    return true;
  } catch {
    return false;
  }
}
```

#### Do

```js
function isUrl(text) {
  return URL.canParse(text);
}
```


### JSX

#### Don't

```jsx
function isUrl(text) {
  try {
    new URL(text);
    return true;
  } catch {
    return false;
  }
}
```

#### Do

```jsx
function isUrl(text) {
  return URL.canParse(text);
}
```


### TypeScript

#### Don't

```ts
function isUrl(text) {
  try {
    new URL(text);
    return true;
  } catch {
    return false;
  }
}
```

#### Do

```ts
function isUrl(text) {
  return URL.canParse(text);
}
```


### TSX

#### Don't

```tsx
function isUrl(text) {
  try {
    new URL(text);
    return true;
  } catch {
    return false;
  }
}
```

#### Do

```tsx
function isUrl(text) {
  return URL.canParse(text);
}
```
