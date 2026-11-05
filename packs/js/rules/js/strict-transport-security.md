---
id: js/strict-transport-security
title: "Keep the HSTS header"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "HSTS commits the domain to HTTPS for its lifetime; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/strict-transport-security]
since: 0.5.0
---

helmet is not configured with `hsts: false`.

**Why:** HSTS makes browsers refuse plain HTTP to the site.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(hsts|strictTransportSecurity)["']?$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: Without HSTS, a first visit over HTTP can be intercepted
fix:
- remove
```

## Examples

### JavaScript

#### Don't

```js
app.use(helmet({ hsts: false }));
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
app.use(helmet({ hsts: false }));
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
app.use(helmet({ hsts: false }));
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
app.use(helmet({ hsts: false }));
```

#### Do

```tsx
app.use(helmet());
```

#### Fixed

```tsx
app.use(helmet({  }));
```
