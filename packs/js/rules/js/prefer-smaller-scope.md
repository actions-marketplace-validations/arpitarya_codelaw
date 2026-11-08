---
id: js/prefer-smaller-scope
title: "A variable declared in the block that uses it"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-smaller-scope]
since: 0.5.0
---

Two shapes are reported at the name. A `let` declared without a value, assigned once by a `name = value;` statement inside a nested block, and used only in that block after the assignment: the fix removes the declaration and writes `const name = value;`. And a `let` or `const` with a literal value, directly followed by an `if` whose one braced branch holds every use: the fix moves the declaration into that branch. A name used across a function boundary, an exported one, a type annotation, a comment, `with` or `eval` leave it alone (or unfixed). TypeScript type aliases are not checked.

**Why:** A variable declared where it is used is read in one place and can't leak into code that doesn't need it.

```codelaw
native: js/prefer-smaller-scope
```

## Examples

### JavaScript

#### Don't

```js
function f(ready) {
  let value;
  if (ready) {
    value = compute();
    log(value);
  }
}
```

#### Do

```js
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```

#### Fixed

```js
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```


### JSX

#### Don't

```jsx
function f(ready) {
  let value;
  if (ready) {
    value = compute();
    log(value);
  }
}
```

#### Do

```jsx
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```

#### Fixed

```jsx
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```


### TypeScript

#### Don't

```ts
function f(ready) {
  let value;
  if (ready) {
    value = compute();
    log(value);
  }
}
```

#### Do

```ts
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```

#### Fixed

```ts
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```


### TSX

#### Don't

```tsx
function f(ready) {
  let value;
  if (ready) {
    value = compute();
    log(value);
  }
}
```

#### Do

```tsx
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```

#### Fixed

```tsx
function f(ready) {
  if (ready) {
    const value = compute();
    log(value);
  }
}
```
