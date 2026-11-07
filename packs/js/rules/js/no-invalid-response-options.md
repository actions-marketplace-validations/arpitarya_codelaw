---
id: js/no-invalid-response-options
title: "No body on a Response whose status forbids one"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "dropping the body is what the status asks for; if the status was the mistake, change it instead"
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-response-options]
since: 0.5.0
---

`new Response(body, { status })` throws a `TypeError` when the status is 204, 205 or 304 and the body is not `null` or `undefined` — even an empty string. A literal body (a string, a template, a number) with one of those statuses is reported, and the fix passes `undefined`. Upstream also checks statuses outside 200–599 and `Response.redirect`'s; those need the right status, which only the author has.

**Why:** The call looks harmless and throws every time it runs.

```codelaw
match:
  kind: new_expression
  pattern: "new Response($B, $O)"
  has:
    field: arguments
    all:
      - has: { nthChild: 1, any: [{ kind: string }, { kind: template_string }, { kind: number }], pattern: $BODY }
      - has: { nthChild: 2, kind: object, has: { kind: pair, all: [{ has: { field: key, regex: '^["'']?status["'']?$' } }, { has: { field: value, kind: number, regex: '^(204|205|304)$' } }] } }
message: "A response with this status has no body; it throws"
fix:
  - replace: { target: $BODY, with: undefined }
```

## Examples

### JavaScript

#### Don't

```js
return new Response("", { status: 204 });
```

#### Do

```js
return new Response(undefined, { status: 204 });
```


### JSX

#### Don't

```jsx
return new Response("", { status: 204 });
```

#### Do

```jsx
return new Response(undefined, { status: 204 });
```


### TypeScript

#### Don't

```ts
return new Response("", { status: 204 });
```

#### Do

```ts
return new Response(undefined, { status: 204 });
```


### TSX

#### Don't

```tsx
return new Response("", { status: 204 });
```

#### Do

```tsx
return new Response(undefined, { status: 204 });
```
