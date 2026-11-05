---
id: sec-js/helmet-config
title: "helmet is not given false for a header it sets"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "deleting the `false` pair makes helmet fall back to its default for that header, which the reviewer checks against the app's inline scripts"
stability: preview
security: { cwe: [693], owasp: ["A02:2025"], severity: medium }
since: 0.3.0
---

`helmet({ … })` does not set `contentSecurityPolicy`, `frameguard`, `hsts`, `noSniff`, `xssFilter` or the cross-origin policies to `false`.

**Why:** Each of these headers closes one class of attack (injected scripts, clickjacking, downgrade to HTTP, MIME sniffing); turning one off opens it again (CWE-693).

```codelaw
match:
  all:
    - pattern: helmet($$$ARGS)
    - has:
        kind: pair
        pattern: $HDR
        stopBy: end
        regex: '^["'']?(contentSecurityPolicy|frameguard|hsts|noSniff|xssFilter|crossOriginOpenerPolicy|crossOriginEmbedderPolicy|crossOriginResourcePolicy|referrerPolicy)["'']?\s*:\s*false\s*$'
message: "`helmet` is told not to send one of its headers"
fix:
  - remove: $HDR
```

## How to fix

1. Keep the header on and tune it: give `contentSecurityPolicy` the directives the app needs.
2. If a header must go for one route, set it off for that route only, with a comment that says why.

## Examples

### JavaScript

#### Don't

```js
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Fixed

```js
app.use(helmet({  }));
```

#### Do

```js
app.use(helmet({ contentSecurityPolicy: { directives: { defaultSrc: ["'self'"] } } }));
```

### JSX

#### Don't

```jsx
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Fixed

```jsx
app.use(helmet({  }));
```

#### Do

```jsx
app.use(helmet({ contentSecurityPolicy: { directives: { defaultSrc: ["'self'"] } } }));
```

### TypeScript

#### Don't

```ts
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Fixed

```ts
app.use(helmet({  }));
```

#### Do

```ts
app.use(helmet({ contentSecurityPolicy: { directives: { defaultSrc: ["'self'"] } } }));
```

### TSX

#### Don't

```tsx
app.use(helmet({ contentSecurityPolicy: false }));
```

#### Fixed

```tsx
app.use(helmet({  }));
```

#### Do

```tsx
app.use(helmet({ contentSecurityPolicy: { directives: { defaultSrc: ["'self'"] } } }));
```
