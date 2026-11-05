---
id: js/no-referrer-policy
title: "A strict Referrer-Policy"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "analytics that read the referrer may change; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-referrer-policy]
since: 0.5.0
---

helmet is not configured with `referrerPolicy: false` or a leaking policy (`unsafe-url`, `no-referrer-when-downgrade`).

**Why:** Full URLs, with tokens in their query strings, reach every linked site.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(referrerPolicy)["']?$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$|unsafe-url|no-referrer-when-downgrade
message: This referrer policy leaks full URLs to other sites
fix:
- remove
```

## Examples

### JavaScript

#### Don't

```js
app.use(helmet({ referrerPolicy: false }));
```

#### Do

```js
app.use(helmet());
```

#### Fixed

```js
app.use(helmet({  }));
```

### JSX

#### Don't

```jsx
app.use(helmet({ referrerPolicy: false }));
```

#### Do

```jsx
app.use(helmet());
```

#### Fixed

```jsx
app.use(helmet({  }));
```

### TypeScript

#### Don't

```ts
app.use(helmet({ referrerPolicy: false }));
```

#### Do

```ts
app.use(helmet());
```

#### Fixed

```ts
app.use(helmet({  }));
```

### TSX

#### Don't

```tsx
app.use(helmet({ referrerPolicy: false }));
```

#### Do

```tsx
app.use(helmet());
```

#### Fixed

```tsx
app.use(helmet({  }));
```
