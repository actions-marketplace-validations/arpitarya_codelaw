---
id: js/try-complexity
title: "A try block holds straight-line code"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which statements may throw, and so stay in the `try`, is the author's call"
fix_exception: restructure
options: { max: { type: "integer", default: 1, description: "the highest cyclomatic complexity a try block may have" } }
stability: preview
ported_from: [eslint-plugin-unicorn/try-complexity]
since: 0.5.0
---

A `try` block's cyclomatic complexity — one plus each `if`, loop, `case`, `catch`, `?:` and `&&`/`||`/`??` in it, not counting nested functions — is at most `max` (1): decisions go before or after the `try`, which holds only the code that can throw.

**Why:** In a large `try` it is unclear which operation the `catch` is for.

```codelaw
native: js/try-complexity
```

## How to fix

1. Move the decisions (`if`, loops, `?:`) before or after the `try`, so it holds only the call that can throw.

## Examples

### JavaScript

#### Don't

```js
try {
  if (ready) {
    load();
  }
} catch (error) {
  report(error);
}
```

#### Do

```js
if (ready) {
  try {
    load();
  } catch (error) {
    report(error);
  }
}
```


### JSX

#### Don't

```jsx
try {
  if (ready) {
    load();
  }
} catch (error) {
  report(error);
}
```

#### Do

```jsx
if (ready) {
  try {
    load();
  } catch (error) {
    report(error);
  }
}
```


### TypeScript

#### Don't

```ts
try {
  if (ready) {
    load();
  }
} catch (error) {
  report(error);
}
```

#### Do

```ts
if (ready) {
  try {
    load();
  } catch (error) {
    report(error);
  }
}
```


### TSX

#### Don't

```tsx
try {
  if (ready) {
    load();
  }
} catch (error) {
  report(error);
}
```

#### Do

```tsx
if (ready) {
  try {
    load();
  } catch (error) {
    report(error);
  }
}
```
