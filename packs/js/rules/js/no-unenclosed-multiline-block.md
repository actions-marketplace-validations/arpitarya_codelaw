---
id: js/no-unenclosed-multiline-block
title: "No statement indented as if it were in a braceless block"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the statement belongs in the block (add braces) or after it (dedent) is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-unenclosed-multiline-block]
since: 0.5.0
---

After an `if`, `for` or `while` whose body is a single statement without braces on its own line, a following statement at the same indentation as that body looks like part of the block but always runs. It is reported.

**Why:** The indentation tells the reader one thing and the code does another.

```codelaw
native: js/no-unenclosed-multiline-block
```

## How to fix

1. If the statement belongs to the block, wrap the body in braces.
2. If it doesn't, dedent it to the level of the `if` or loop.

## Examples

### JavaScript

#### Don't

```js
if (ready)
  start();
  notify();
```

#### Do

```js
if (ready) {
  start();
  notify();
}
```


### JSX

#### Don't

```jsx
if (ready)
  start();
  notify();
```

#### Do

```jsx
if (ready) {
  start();
  notify();
}
```


### TypeScript

#### Don't

```ts
if (ready)
  start();
  notify();
```

#### Do

```ts
if (ready) {
  start();
  notify();
}
```


### TSX

#### Don't

```tsx
if (ready)
  start();
  notify();
```

#### Do

```tsx
if (ready) {
  start();
  notify();
}
```
