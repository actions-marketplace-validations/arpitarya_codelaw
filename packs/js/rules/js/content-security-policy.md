---
id: js/content-security-policy
title: "Keep the Content-Security-Policy header"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "a strict policy can break inline scripts, which must be moved first; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/content-security-policy]
since: 0.5.0
---

helmet is not configured with `contentSecurityPolicy: false`.

**Why:** Without a CSP, injected script runs with the page's full rights.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(contentSecurityPolicy)["']?$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: Turning off the CSP removes the main defence against XSS
fix:
- remove
```

## Examples

### JavaScript

#### Don't

```js
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Do

```js
app.use(helmet({  }));
```

### JSX

#### Don't

```jsx
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Do

```jsx
app.use(helmet({  }));
```

### TypeScript

#### Don't

```ts
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Do

```ts
app.use(helmet({  }));
```

### TSX

#### Don't

```tsx
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Do

```tsx
app.use(helmet({  }));
```
