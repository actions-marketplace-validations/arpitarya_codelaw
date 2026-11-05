---
id: js/throw-new-error
title: "throw new Error, not Error()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/throw-new-error]
since: 0.5.0
---

An error is thrown with `new` (`throw new Error(…)`).

**Why:** Consistency with every other constructor; `Error()` works without `new` only by accident of history.

```codelaw
match:
  pattern: "$E($$$A)"
  inside: { kind: throw_statement }
constraints:
  E: { regex: '^([A-Z][A-Za-z]*)?Error$' }
message: "Use `new $E(...)`"
fix: "new $E($$$A)"
```

## Examples

### JavaScript

#### Don't

```js
throw Error("bad input");
```

#### Do

```js
throw new Error("bad input");
```

### JSX

#### Don't

```jsx
throw Error("bad input");
```

#### Do

```jsx
throw new Error("bad input");
```

### TypeScript

#### Don't

```ts
throw Error("bad input");
```

#### Do

```ts
throw new Error("bad input");
```

### TSX

#### Don't

```tsx
throw Error("bad input");
```

#### Do

```tsx
throw new Error("bad input");
```
