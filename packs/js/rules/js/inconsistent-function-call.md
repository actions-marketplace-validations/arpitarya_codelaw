---
id: js/inconsistent-function-call
title: "No function called both with and without `new`"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the function is a constructor or a plain function is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/inconsistent-function-call]
since: 0.5.0
---

A function the file declares (a name, not a member) that is called with `new` in one place and without it in another is reported at the second kind of call, naming the line of the other. Each function is reported once; undeclared globals (`Date`, `Number`) are left alone.

**Why:** A function is either a constructor or not: one of the two calls does something other than the author thinks.

```codelaw
native: js/inconsistent-function-call
```

## How to fix

1. Call it the same way everywhere: with `new` if it builds an object, without if it returns one.

## Examples

### JavaScript

#### Don't

```js
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = Point(2);
```

#### Do

```js
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = new Point(2);
```


### JSX

#### Don't

```jsx
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = Point(2);
```

#### Do

```jsx
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = new Point(2);
```


### TypeScript

#### Don't

```ts
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = Point(2);
```

#### Do

```ts
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = new Point(2);
```


### TSX

#### Don't

```tsx
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = Point(2);
```

#### Do

```tsx
function Point(x) {
  this.x = x;
}
const a = new Point(1);
const b = new Point(2);
```
