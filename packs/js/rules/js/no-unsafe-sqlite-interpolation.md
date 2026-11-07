---
id: js/no-unsafe-sqlite-interpolation
title: "No values interpolated into an SQL statement"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the parameters and their binding are the author's"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-unicorn/no-unsafe-sqlite-interpolation]
since: 0.5.0
---

An SQL statement passed to `prepare()` or `exec()` (`node:sqlite`, better-sqlite3 and kin) is not a template literal with `${…}` in it; values are bound as parameters.

**Why:** An interpolated value becomes SQL, so a quote in it changes the statement: SQL injection.

```codelaw
match:
  pattern: "$DB.$M($T)"
constraints:
  M: { regex: '^(prepare|exec)$' }
  T:
    kind: template_string
    regex: '^`\s*(?i:select|insert|update|delete|create|drop|alter|replace|with|pragma)\b'
    has: { kind: template_substitution }
message: "Bind the values as parameters instead of interpolating them"
```

## How to fix

1. Write `?` (or `:name`) where each value goes and pass the values to `run`, `get` or `all`.

## Examples

### JavaScript

#### Don't

```js
const row = db.prepare(`SELECT * FROM users WHERE id = ${id}`).get();
```

#### Do

```js
const row = db.prepare("SELECT * FROM users WHERE id = ?").get(id);
```


### JSX

#### Don't

```jsx
const row = db.prepare(`SELECT * FROM users WHERE id = ${id}`).get();
```

#### Do

```jsx
const row = db.prepare("SELECT * FROM users WHERE id = ?").get(id);
```


### TypeScript

#### Don't

```ts
const row = db.prepare(`SELECT * FROM users WHERE id = ${id}`).get();
```

#### Do

```ts
const row = db.prepare("SELECT * FROM users WHERE id = ?").get(id);
```


### TSX

#### Don't

```tsx
const row = db.prepare(`SELECT * FROM users WHERE id = ${id}`).get();
```

#### Do

```tsx
const row = db.prepare("SELECT * FROM users WHERE id = ?").get(id);
```
