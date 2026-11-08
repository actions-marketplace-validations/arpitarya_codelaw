---
id: js/no-useless-length-check
title: "No length check before every() or some()"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-length-check]
since: 0.5.0
---

An array's length is not checked before `every()` or `some()` when the check changes nothing: `arr.length === 0 || arr.every(f)` is `arr.every(f)` (true for an empty array), and `arr.length > 0 && arr.some(f)` is `arr.some(f)` (false for one).

**Why:** The check repeats what the method already returns for an empty array.

```codelaw
match:
  any:
    - pattern: "$A.length === 0 || $A.every($$$F)"
      has: { field: right, pattern: $R }
    - pattern: "$A.length !== 0 && $A.some($$$F)"
      has: { field: right, pattern: $R }
    - pattern: "$A.length > 0 && $A.some($$$F)"
      has: { field: right, pattern: $R }
message: "The length check is redundant"
fix: "$R"
```

## Examples

### JavaScript

#### Don't

```js
const ok = items.length === 0 || items.every(isValid);
const any = items.length > 0 && items.some(isValid);
```

#### Do

```js
const ok = items.every(isValid);
const any = items.some(isValid);
```


### JSX

#### Don't

```jsx
const ok = items.length === 0 || items.every(isValid);
const any = items.length > 0 && items.some(isValid);
```

#### Do

```jsx
const ok = items.every(isValid);
const any = items.some(isValid);
```


### TypeScript

#### Don't

```ts
const ok = items.length === 0 || items.every(isValid);
const any = items.length > 0 && items.some(isValid);
```

#### Do

```ts
const ok = items.every(isValid);
const any = items.some(isValid);
```


### TSX

#### Don't

```tsx
const ok = items.length === 0 || items.every(isValid);
const any = items.length > 0 && items.some(isValid);
```

#### Do

```tsx
const ok = items.every(isValid);
const any = items.some(isValid);
```
