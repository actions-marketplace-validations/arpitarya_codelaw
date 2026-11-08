---
id: js/getter-return
title: "A getter returns a value on every path"
languages: [javascript, jsx]
fix: suggest
fix_reason: "the path that falls off the end gets `return undefined;`, which it already returns; the reviewer confirms that is the value meant"
fix_exception: value
stability: preview
ported_from: [eslint/getter-return]
since: 0.1.0
---

Every path through a getter ends in `return <value>` or a `throw`.

**Why:** A getter that falls off its end returns `undefined`, which is almost always a forgotten
`return`.

```codelaw
native: js/getter-return
```

## How to fix

1. Find the path that ends without a `return`.
2. Return the value the property should have there, or throw.

## Examples

### JavaScript

#### Don't

```js
const box = {
  get size() {
    if (this.items) {
      return this.items.length;
    }
  },
};
```

#### Fixed

```js
const box = {
  get size() {
    if (this.items) {
      return this.items.length;
    }
    return undefined;
  },
};
```

#### Do

```js
const box = {
  get size() {
    return this.items ? this.items.length : 0;
  },
};
```

### JSX

#### Don't

```jsx
const box = {
  get size() {
    if (this.items) {
      return this.items.length;
    }
  },
};
```

#### Fixed

```jsx
const box = {
  get size() {
    if (this.items) {
      return this.items.length;
    }
    return undefined;
  },
};
```

#### Do

```jsx
const box = {
  get size() {
    return this.items ? this.items.length : 0;
  },
};
```
