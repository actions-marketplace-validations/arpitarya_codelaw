---
id: js/jsdoc-check-access
title: "@access takes a known level, once"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which access level was meant is the author's choice"
fix_pending: "suggest-choices; needs computed-edit (W-162)"
stability: preview
ported_from: [eslint-plugin-jsdoc/check-access]
since: 0.5.0
---

A JSDoc block's `@access` names `package`, `private`, `protected` or `public`, and a block carries at most one access tag (`@access`, `@package`, `@private`, `@protected`, `@public`).

**Why:** Documentation tools ignore or mis-file a member whose access level is misspelt or stated twice, so the published API says something the author did not mean.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - any:
        - all:
            - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@access\b'
            - not: {regex: '@access[ \t]+(package|private|protected|public)\b'}
        - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(access|package|private|protected|public)\b[\s\S]*\n[ \t]*(\*[ \t]*)?@(access|package|private|protected|public)\b'
message: "`@access` takes package, private, protected or public, and a block holds one access tag; keep the one you mean"
```

## How to fix

1. Decide the member's access level.
2. Keep one tag for it: `@access <level>` with a known level, or the matching `@private`/`@protected`/`@public`/`@package` alone.

## Examples

### JavaScript

#### Don't

```js
/**
 * Clears the cache.
 * @access internal
 */
function clearCache() {}
```

#### Do

```js
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

### JSX

#### Don't

```jsx
/**
 * Clears the cache.
 * @access internal
 */
function clearCache() {}
```

#### Do

```jsx
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

### TypeScript

#### Don't

```ts
/**
 * Clears the cache.
 * @access internal
 */
function clearCache() {}
```

#### Do

```ts
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

### TSX

#### Don't

```tsx
/**
 * Clears the cache.
 * @access internal
 */
function clearCache() {}
```

#### Do

```tsx
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```
