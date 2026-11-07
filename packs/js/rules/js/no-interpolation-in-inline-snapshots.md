---
id: js/no-interpolation-in-inline-snapshots
title: "No interpolation in an inline snapshot"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the snapshot text is the author's"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-sonarjs/no-interpolation-in-inline-snapshots]
since: 0.5.0
---

The template passed to `toMatchInlineSnapshot` or `toThrowErrorMatchingInlineSnapshot` has no `${…}`.

**Why:** The test runner writes and compares the snapshot as literal text; an interpolation is overwritten on the next update.

```codelaw
match:
  pattern: "$E.$M($S)"
  has: { field: arguments, has: { kind: template_string, has: { kind: template_substitution } } }
constraints:
  M: { regex: '^(toMatchInlineSnapshot|toThrowErrorMatchingInlineSnapshot)$' }
message: "An inline snapshot is literal text; remove the interpolation"
```

## How to fix

1. Write the value into the snapshot, or check the dynamic part with a separate assertion.

## Examples

### JavaScript

#### Don't

```js
expect(render(user)).toMatchInlineSnapshot(`<p>${user.name}</p>`);
```

#### Do

```js
expect(render(user)).toMatchInlineSnapshot(`<p>Ada</p>`);
```


### JSX

#### Don't

```jsx
expect(render(user)).toMatchInlineSnapshot(`<p>${user.name}</p>`);
```

#### Do

```jsx
expect(render(user)).toMatchInlineSnapshot(`<p>Ada</p>`);
```


### TypeScript

#### Don't

```ts
expect(render(user)).toMatchInlineSnapshot(`<p>${user.name}</p>`);
```

#### Do

```ts
expect(render(user)).toMatchInlineSnapshot(`<p>Ada</p>`);
```


### TSX

#### Don't

```tsx
expect(render(user)).toMatchInlineSnapshot(`<p>${user.name}</p>`);
```

#### Do

```tsx
expect(render(user)).toMatchInlineSnapshot(`<p>Ada</p>`);
```
