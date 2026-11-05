---
id: js/prefer-optional-catch-binding
title: "Omit an unused catch binding"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-optional-catch-binding]
since: 0.5.0
---

A `catch` clause whose parameter is never used omits it.

**Why:** An unused binding is noise; the optional form says the error is deliberately ignored.

```codelaw
match:
  kind: catch_clause
  has:
    field: parameter
    kind: identifier
    pattern: $E
  all:
  - has:
      field: body
      pattern: $B
where:
  $E: unused
message: '`$E` is not used; write `catch {`'
fix: catch $B
```

## Examples

### JavaScript

#### Don't

```js
try {
  parse();
} catch (error) {
  fallback();
}
```

#### Do

```js
try {
  parse();
} catch {
  fallback();
}
```

### JSX

#### Don't

```jsx
try {
  parse();
} catch (error) {
  fallback();
}
```

#### Do

```jsx
try {
  parse();
} catch {
  fallback();
}
```

### TypeScript

#### Don't

```ts
try {
  parse();
} catch (error) {
  fallback();
}
```

#### Do

```ts
try {
  parse();
} catch {
  fallback();
}
```

### TSX

#### Don't

```tsx
try {
  parse();
} catch (error) {
  fallback();
}
```

#### Do

```tsx
try {
  parse();
} catch {
  fallback();
}
```
