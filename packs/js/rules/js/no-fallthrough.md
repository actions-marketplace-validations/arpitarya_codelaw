---
id: js/no-fallthrough
title: "A switch case does not fall through into the next by accident"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the choices are `break;` ending the case, which stops the fall-through, or a `// falls through` comment saying it is meant, which keeps behaviour; only the author knows which"
stability: preview
ported_from: [eslint/no-fallthrough]
since: 0.1.0
---

A `case` with statements ends in `break`, `return`, `throw` or `continue`, or says `// falls
through` before the next case.

**Why:** Falling through is easy to do by accident and hard to see in review.

```codelaw
native: js/no-fallthrough
```

## How to fix

1. If the case should stop, end it with `break` (or `return`).
2. If it should fall through, say so with a `// falls through` comment before the next case.

## Examples

### JavaScript

#### Don't

```js
switch (kind) {
  case "a":
    first();
  case "b":
    second();
    break;
}
```

#### Fixed (break)

```js
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

#### Fixed (falls through)

```js
switch (kind) {
  case "a":
    first();
    // falls through
  case "b":
    second();
    break;
}
```

#### Do

```js
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

```js
switch (kind) {
  case "a":
    first();
  // falls through
  case "b":
    second();
    break;
}
```

### JSX

#### Don't

```jsx
switch (kind) {
  case "a":
    first();
  case "b":
    second();
    break;
}
```

#### Fixed (break)

```jsx
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

#### Fixed (falls through)

```jsx
switch (kind) {
  case "a":
    first();
    // falls through
  case "b":
    second();
    break;
}
```

#### Do

```jsx
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

```jsx
switch (kind) {
  case "a":
    first();
  // falls through
  case "b":
    second();
    break;
}
```

### TypeScript

#### Don't

```ts
switch (kind) {
  case "a":
    first();
  case "b":
    second();
    break;
}
```

#### Fixed (break)

```ts
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

#### Fixed (falls through)

```ts
switch (kind) {
  case "a":
    first();
    // falls through
  case "b":
    second();
    break;
}
```

#### Do

```ts
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

```ts
switch (kind) {
  case "a":
    first();
  // falls through
  case "b":
    second();
    break;
}
```

### TSX

#### Don't

```tsx
switch (kind) {
  case "a":
    first();
  case "b":
    second();
    break;
}
```

#### Fixed (break)

```tsx
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

#### Fixed (falls through)

```tsx
switch (kind) {
  case "a":
    first();
    // falls through
  case "b":
    second();
    break;
}
```

#### Do

```tsx
switch (kind) {
  case "a":
    first();
    break;
  case "b":
    second();
    break;
}
```

```tsx
switch (kind) {
  case "a":
    first();
  // falls through
  case "b":
    second();
    break;
}
```
