---
id: js/accessor-pairs
title: "A setter has a getter"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the getter returns is the author's choice"
stability: preview
ported_from: [eslint/accessor-pairs]
since: 0.5.0
---

An object or class that defines a setter for a property also defines its getter (the upstream default checks setters only).

**Why:** A write-only property reads back as `undefined`, which is almost never what the author meant.

```codelaw

match:
  kind: method_definition
  regex: '^(static\s+)?set\s'
  has: { field: name, pattern: $N }
  not:
    inside:
      any: [{kind: class_body}, {kind: object}]
      has: { kind: method_definition, regex: '^(static\s+)?get\s', has: { field: name, pattern: $N } }
message: "`$N` can be set but not read; add a getter"
```

## How to fix

1. Add a getter for the same name that returns the stored value.

## Examples

### JavaScript

#### Don't

```js
const box = {
  set value(v) {
    this._v = v;
  },
};
```

#### Do

```js
const box = {
  get value() {
    return this._v;
  },
  set value(v) {
    this._v = v;
  },
};
```

### JSX

#### Don't

```jsx
const box = {
  set value(v) {
    this._v = v;
  },
};
```

#### Do

```jsx
const box = {
  get value() {
    return this._v;
  },
  set value(v) {
    this._v = v;
  },
};
```

### TypeScript

#### Don't

```ts
const box = {
  set value(v) {
    this._v = v;
  },
};
```

#### Do

```ts
const box = {
  get value() {
    return this._v;
  },
  set value(v) {
    this._v = v;
  },
};
```

### TSX

#### Don't

```tsx
const box = {
  set value(v) {
    this._v = v;
  },
};
```

#### Do

```tsx
const box = {
  get value() {
    return this._v;
  },
  set value(v) {
    this._v = v;
  },
};
```
