---
id: js/no-useless-delete-check
title: "No existence check before a delete"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-delete-check]
since: 0.5.0
---

A key is deleted without first checking it is there: `if (map.has(k)) map.delete(k)` is `map.delete(k)`, and `if (k in obj) delete obj[k]` is `delete obj[k]`.

**Why:** Deleting a missing key already does nothing; the check only repeats the lookup.

```codelaw
match:
  any:
    - pattern: "if ($M.has($K)) $M.delete($K);"
      not: { has: { kind: else_clause } }
      all: [{ pattern: $MAP }]
    - pattern: "if ($M.has($K)) { $M.delete($K); }"
      not: { has: { kind: else_clause } }
      all: [{ pattern: $MAP }]
    - pattern: "if ($K in $O) delete $O[$K];"
      not: { has: { kind: else_clause } }
      all: [{ pattern: $OBJ }]
    - pattern: "if ($K in $O) { delete $O[$K]; }"
      not: { has: { kind: else_clause } }
      all: [{ pattern: $OBJ }]
message: "Delete without the check"
fix:
  - replace: { target: $MAP, with: "$M.delete($K);" }
  - replace: { target: $OBJ, with: "delete $O[$K];" }
```

## Examples

### JavaScript

#### Don't

```js
if (cache.has(key)) cache.delete(key);
```

#### Do

```js
cache.delete(key);
```


### JSX

#### Don't

```jsx
if (cache.has(key)) cache.delete(key);
```

#### Do

```jsx
cache.delete(key);
```


### TypeScript

#### Don't

```ts
if (cache.has(key)) cache.delete(key);
```

#### Do

```ts
cache.delete(key);
```


### TSX

#### Don't

```tsx
if (cache.has(key)) cache.delete(key);
```

#### Do

```tsx
cache.delete(key);
```
