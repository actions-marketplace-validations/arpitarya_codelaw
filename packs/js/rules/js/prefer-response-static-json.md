---
id: js/prefer-response-static-json
title: "Response.json rather than new Response(JSON.stringify())"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the response gains a `Content-Type: application/json` header"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-response-static-json]
since: 0.5.0
---

`new Response(JSON.stringify(x))` is written `Response.json(x)`.

**Why:** `Response.json` also sets the `Content-Type` header.

```codelaw
match:
  pattern: "new Response(JSON.stringify($X))"
message: "Use `Response.json()`"
fix: "Response.json($X)"
```

## Examples

### JavaScript

#### Don't

```js
return new Response(JSON.stringify(body));
```

#### Do

```js
return Response.json(body);
```

### JSX

#### Don't

```jsx
return new Response(JSON.stringify(body));
```

#### Do

```jsx
return Response.json(body);
```

### TypeScript

#### Don't

```ts
return new Response(JSON.stringify(body));
```

#### Do

```ts
return Response.json(body);
```

### TSX

#### Don't

```tsx
return new Response(JSON.stringify(body));
```

#### Do

```tsx
return Response.json(body);
```
