---
id: js/prefer-dispose
title: "Dispose a resource with `using`, not a `try`/`finally`"
languages: [javascript, jsx]
fix: suggest
fix_reason: "the value must have `Symbol.dispose` (`Symbol.asyncDispose` for `await using`), which only the author knows"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-dispose]
since: 0.5.2
---

A `try`/`finally` with no `catch` whose `finally` only lets go of resources declared with `const` just before it (`file.close()`, `await connection.release()`) is a `using` declaration. A `finally` statement counts when it is a call of `close`, `dispose`, `destroy`, `end`, `release`, `disconnect`, `unlock` or `free` on one of those resources, awaited or not. The resources must be disposed once each, in the reverse of their declaration order; a name that is read after the `try` is not reported, since the block that holds `using` would end its scope. The suggestion is the block `{ using … ; body }`, with `await using` for an awaited disposal. Behaviour of eslint-plugin-unicorn 79.0.0's `prefer-dispose`, read and rewritten; with no type information it cannot know the value is disposable (an object with only `.close()` throws a `TypeError` under `using`), so the edit is a suggestion and the rule is off by default. JavaScript and JSX only: the pinned TypeScript grammar does not parse `using` (W-190, step 2).

**Why:** The block disposes in reverse order by itself, and keeps the original error when a disposer throws; a hand-written `finally` can replace it.

```codelaw
native: js/prefer-dispose
```

## How to fix

1. Check the value implements `Symbol.dispose` (or `Symbol.asyncDispose`); add it if the class is yours.
2. Accept the suggestion.

## Examples

### JavaScript

#### Don't

```js
const file = open();
try {
  read(file);
} finally {
  file.close();
}
```

#### Do

```js
{
  using file = open();
  read(file);
}
```

### JSX

#### Don't

```jsx
const file = open();
try {
  read(file);
} finally {
  file.close();
}
```

#### Do

```jsx
{
  using file = open();
  read(file);
}
```
