---
id: js/no-mime-sniff
title: "Keep X-Content-Type-Options"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "none usually; the header only stops sniffing; the change closes access that something may rely on"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-mime-sniff]
since: 0.5.0
---

helmet is not configured with `noSniff: false`.

**Why:** Without `nosniff`, a browser can treat a user-supplied file as script or HTML.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(noSniff|xContentTypeOptions)["']?$
  all:
  - has:
      field: value
      pattern: $V
      regex: ^false$
message: Browsers may sniff and run uploaded files as script; keep `nosniff`
fix:
- remove
```

## Examples

### JavaScript

#### Don't

```js
app.use(helmet({ noSniff: false }));
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
app.use(helmet({ noSniff: false }));
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
app.use(helmet({ noSniff: false }));
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
app.use(helmet({ noSniff: false }));
```

#### Do

```tsx
app.use(helmet());
```

#### Fixed

```tsx
app.use(helmet({  }));
```
