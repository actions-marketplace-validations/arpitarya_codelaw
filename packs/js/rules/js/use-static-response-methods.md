---
id: js/use-static-response-methods
title: "`Response.json()` instead of `new Response(JSON.stringify())`"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Response.json()` sets `content-type: application/json` itself, which changes the headers of a response that set another type"
stability: preview
ported_from: [biome/suspicious/useStaticResponseMethods]
since: 0.5.1
---

`new Response(JSON.stringify(data))` and `new Response(JSON.stringify(data), init)` are written `Response.json(data)` and `Response.json(data, init)`. Behaviour of Biome's `useStaticResponseMethods` for the JSON form.

**Why:** The static method says what the response is, sets the JSON content type, and drops the manual serialisation.

```codelaw
match:
  any:
    - all: [{ pattern: $A }, { pattern: new Response(JSON.stringify($X)) }]
    - all: [{ pattern: $B }, { pattern: "new Response(JSON.stringify($X), $INIT)" }]
message: "Use `Response.json()` instead of `new Response(JSON.stringify())`"
fix:
  - replace: { target: $A, with: "Response.json($X)" }
  - replace: { target: $B, with: "Response.json($X, $INIT)" }
```

## Examples

### JavaScript

#### Don't

```js
const res = new Response(JSON.stringify(data));
```

#### Do

```js
const res = Response.json(data);
```

#### Fixed

```js
const res = Response.json(data);
```

### JSX

#### Don't

```jsx
const res = new Response(JSON.stringify(data));
```

#### Do

```jsx
const res = Response.json(data);
```

#### Fixed

```jsx
const res = Response.json(data);
```

### TypeScript

#### Don't

```ts
const res = new Response(JSON.stringify(data));
```

#### Do

```ts
const res = Response.json(data);
```

#### Fixed

```ts
const res = Response.json(data);
```

### TSX

#### Don't

```tsx
const res = new Response(JSON.stringify(data));
```

#### Do

```tsx
const res = Response.json(data);
```

#### Fixed

```tsx
const res = Response.json(data);
```
