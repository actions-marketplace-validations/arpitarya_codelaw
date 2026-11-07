---
id: js/jsdoc-check-access
title: "@access takes a known level, once"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "which access level was meant is the author's choice, so each of the four is offered for an unknown `@access` value; a block with two access tags keeps the one the author means, by hand"
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
            - pattern: $BAD
        - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(access|package|private|protected|public)\b[\s\S]*\n[ \t]*(\*[ \t]*)?@(access|package|private|protected|public)\b'
message: "`@access` takes package, private, protected or public, and a block holds one access tag; keep the one you mean"
fix:
  choices:
    - label: package
      fix:
        - edit-text: { target: $BAD, pattern: '@access(?:[ \t]+[^\s*]+)?', replace: '@access package' }
    - label: private
      fix:
        - edit-text: { target: $BAD, pattern: '@access(?:[ \t]+[^\s*]+)?', replace: '@access private' }
    - label: protected
      fix:
        - edit-text: { target: $BAD, pattern: '@access(?:[ \t]+[^\s*]+)?', replace: '@access protected' }
    - label: public
      fix:
        - edit-text: { target: $BAD, pattern: '@access(?:[ \t]+[^\s*]+)?', replace: '@access public' }
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

#### Fixed (package)

```js
/**
 * Clears the cache.
 * @access package
 */
function clearCache() {}
```

#### Fixed (private)

```js
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

#### Fixed (protected)

```js
/**
 * Clears the cache.
 * @access protected
 */
function clearCache() {}
```

#### Fixed (public)

```js
/**
 * Clears the cache.
 * @access public
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

#### Fixed (package)

```jsx
/**
 * Clears the cache.
 * @access package
 */
function clearCache() {}
```

#### Fixed (private)

```jsx
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

#### Fixed (protected)

```jsx
/**
 * Clears the cache.
 * @access protected
 */
function clearCache() {}
```

#### Fixed (public)

```jsx
/**
 * Clears the cache.
 * @access public
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

#### Fixed (package)

```ts
/**
 * Clears the cache.
 * @access package
 */
function clearCache() {}
```

#### Fixed (private)

```ts
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

#### Fixed (protected)

```ts
/**
 * Clears the cache.
 * @access protected
 */
function clearCache() {}
```

#### Fixed (public)

```ts
/**
 * Clears the cache.
 * @access public
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

#### Fixed (package)

```tsx
/**
 * Clears the cache.
 * @access package
 */
function clearCache() {}
```

#### Fixed (private)

```tsx
/**
 * Clears the cache.
 * @access private
 */
function clearCache() {}
```

#### Fixed (protected)

```tsx
/**
 * Clears the cache.
 * @access protected
 */
function clearCache() {}
```

#### Fixed (public)

```tsx
/**
 * Clears the cache.
 * @access public
 */
function clearCache() {}
```
