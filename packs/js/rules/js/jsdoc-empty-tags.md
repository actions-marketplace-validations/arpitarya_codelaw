---
id: js/jsdoc-empty-tags
title: "Flag tags like @private carry no text"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the text after the tag is prose; where it belongs in the description, and how it reads there, is the author's writing"
fix_exception: authored
stability: preview
ported_from: [eslint-plugin-jsdoc/empty-tags]
since: 0.5.0
---

Tags that are flags — `@abstract`, `@async`, `@generator`, `@global`, `@hideconstructor`, `@ignore`, `@inheritdoc`, `@inner`, `@instance`, `@internal`, `@overload`, `@override`, `@package`, `@private`, `@protected`, `@public`, `@readonly`, `@static` — have nothing after them on their line.

**Why:** Documentation tools discard text after a flag tag, so whatever the author wrote there never reaches a reader.

```codelaw
match:
  kind: comment
  all:
    - regex: '^/\*\*[^*/]'
    - regex: '(^/\*\*|\n)[ \t]*(\*[ \t]*)?@(abstract|async|generator|global|hideconstructor|ignore|inheritdoc|inheritDoc|inner|instance|internal|overload|override|package|private|protected|public|readonly|static)[ \t]+[^\s*]'
message: "This flag tag takes no text; move the text into the block's description"
```

## How to fix

1. Move the text after the flag tag into the block's description.
2. Leave the tag alone on its line: `@private`.

## Examples

### JavaScript

#### Don't

```js
/**
 * Resets the counter.
 * @private only the scheduler calls this
 */
function reset() {}
```

#### Do

```js
/**
 * Resets the counter. Only the scheduler calls this.
 * @private
 */
function reset() {}
```

### JSX

#### Don't

```jsx
/**
 * Resets the counter.
 * @private only the scheduler calls this
 */
function reset() {}
```

#### Do

```jsx
/**
 * Resets the counter. Only the scheduler calls this.
 * @private
 */
function reset() {}
```

### TypeScript

#### Don't

```ts
/**
 * Resets the counter.
 * @private only the scheduler calls this
 */
function reset() {}
```

#### Do

```ts
/**
 * Resets the counter. Only the scheduler calls this.
 * @private
 */
function reset() {}
```

### TSX

#### Don't

```tsx
/**
 * Resets the counter.
 * @private only the scheduler calls this
 */
function reset() {}
```

#### Do

```tsx
/**
 * Resets the counter. Only the scheduler calls this.
 * @private
 */
function reset() {}
```
