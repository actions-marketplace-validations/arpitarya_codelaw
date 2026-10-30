---
id: js/no-iterator
title: "No __iterator__"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "moving to `Symbol.iterator` rewrites the iterator"
stability: preview
ported_from: [eslint/no-iterator]
since: 0.5.0
---

The obsolete `__iterator__` property is not used.

**Why:** It is a non-standard SpiderMonkey extension; `Symbol.iterator` is the standard.

```codelaw
match:
  any:
    - kind: member_expression
      has: { field: property, regex: '^__iterator__$' }
    - kind: subscript_expression
      has: { field: index, regex: '^["''`]__iterator__["''`]$' }
message: "Use `Symbol.iterator` instead of `__iterator__`"
```

## How to fix

1. Implement `[Symbol.iterator]()` instead.

## Examples

### JavaScript

#### Don't

```js
Foo.prototype.__iterator__ = function () { return items(this); };
```

#### Do

```js
Foo.prototype[Symbol.iterator] = function () { return items(this); };
```

### JSX

#### Don't

```jsx
Foo.prototype.__iterator__ = function () { return items(this); };
```

#### Do

```jsx
Foo.prototype[Symbol.iterator] = function () { return items(this); };
```

### TypeScript

#### Don't

```ts
Foo.prototype.__iterator__ = function () { return items(this); };
```

#### Do

```ts
Foo.prototype[Symbol.iterator] = function () { return items(this); };
```

### TSX

#### Don't

```tsx
Foo.prototype.__iterator__ = function () { return items(this); };
```

#### Do

```tsx
Foo.prototype[Symbol.iterator] = function () { return items(this); };
```
