---
id: js/no-plusplus
title: "No ++ or --"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "prefix and postfix forms differ in value, so the rewrite needs the author's eye"
fix_pending: "auto-subset"
stability: preview
ported_from: [eslint/no-plusplus]
since: 0.5.0
---

`++` and `--` are not used.

**Why:** Without semicolons, `++` can attach to the wrong line; `+= 1` cannot.

```codelaw
match:
  kind: update_expression
message: "Use `+= 1` or `-= 1` instead of `++` or `--`"
```

## How to fix

1. Write `i += 1` (or `i -= 1`).

## Examples

### JavaScript

#### Don't

```js
count++;
```

#### Do

```js
count += 1;
```

### JSX

#### Don't

```jsx
count++;
```

#### Do

```jsx
count += 1;
```

### TypeScript

#### Don't

```ts
count++;
```

#### Do

```ts
count += 1;
```

### TSX

#### Don't

```tsx
count++;
```

#### Do

```tsx
count += 1;
```
