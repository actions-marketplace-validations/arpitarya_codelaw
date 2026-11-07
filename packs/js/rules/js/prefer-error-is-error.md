---
id: js/prefer-error-is-error
title: "Error.isError() rather than instanceof Error"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-error-is-error]
since: 0.5.0
---

Whether a value is an error is tested with `Error.isError(value)`, not `value instanceof Error`.

**Why:** `instanceof` fails for an error made in another realm (an iframe, a worker, a `vm` context); `Error.isError` checks the internal slot every error has.

```codelaw
match:
  pattern: "$X instanceof Error"
message: "Test with `Error.isError($X)`"
fix: "Error.isError($X)"
```

## Examples

### JavaScript

#### Don't

```js
if (reason instanceof Error) report(reason);
```

#### Do

```js
if (Error.isError(reason)) report(reason);
```


### JSX

#### Don't

```jsx
if (reason instanceof Error) report(reason);
```

#### Do

```jsx
if (Error.isError(reason)) report(reason);
```


### TypeScript

#### Don't

```ts
if (reason instanceof Error) report(reason);
```

#### Do

```ts
if (Error.isError(reason)) report(reason);
```


### TSX

#### Don't

```tsx
if (reason instanceof Error) report(reason);
```

#### Do

```tsx
if (Error.isError(reason)) report(reason);
```
