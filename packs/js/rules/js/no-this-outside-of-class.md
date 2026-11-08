---
id: js/no-this-outside-of-class
title: "No `this` outside a class"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what the code should use instead of `this` (a parameter, a class) is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-this-outside-of-class]
since: 0.5.0
---

`this` is used only where a class gives it a meaning: in a class's constructor, methods and accessors, its field values and static blocks, and in arrow functions inside them. A function with a TypeScript `this` parameter declares what it is bound to. Everywhere else — object-literal methods, plain functions (also ones nested in a method), prototype assignments, the top level — it is reported.

**Why:** Outside a class, `this` is whatever the caller binds, so the same function behaves differently depending on how it is called.

```codelaw
native: js/no-this-outside-of-class
```

## How to fix

1. Pass the object the function works on as a parameter, or make the code a class with methods.
2. Inside a method, turn a nested `function` that reads `this` into an arrow function.

## Examples

### JavaScript

#### Don't

```js
const counter = {
  count: 0,
  increment() {
    this.count += 1;
  },
};
```

#### Do

```js
class Counter {
  count = 0;
  increment() {
    this.count += 1;
  }
}
```


### JSX

#### Don't

```jsx
const counter = {
  count: 0,
  increment() {
    this.count += 1;
  },
};
```

#### Do

```jsx
class Counter {
  count = 0;
  increment() {
    this.count += 1;
  }
}
```


### TypeScript

#### Don't

```ts
const counter = {
  count: 0,
  increment() {
    this.count += 1;
  },
};
```

#### Do

```ts
class Counter {
  count = 0;
  increment() {
    this.count += 1;
  }
}
```


### TSX

#### Don't

```tsx
const counter = {
  count: 0,
  increment() {
    this.count += 1;
  },
};
```

#### Do

```tsx
class Counter {
  count = 0;
  increment() {
    this.count += 1;
  }
}
```
