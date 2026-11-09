---
id: js/no-unnecessary-parameters
title: "No parameter that gets the same value at every call"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the fix writes the value into the function and drops the argument from every call, and only where that is certain (a primitive, or a binding the function can already see); in a TypeScript file, or one with JSDoc types, it is a suggestion, because the signature changes; a parameter with a default that names another parameter, a destructured property, a global and any case a comment is in the way are reported without an edit"
stability: preview
options: { minimumCallCount: { type: "integer", default: 2, description: "the fewest calls outside the function that make it worth checking; calls inside it never count" } }
ported_from: [eslint-plugin-unicorn/no-unnecessary-parameters]
since: 0.5.0
---

A parameter that receives the same value, or always its default, at every known call of the function in this file is reported at its name. The functions followed are those whose every use is a direct call: `function` declarations, `const f = () => …` and `function` expressions, the explicit constructor of a class called only with `new`, and `#private` methods; an exported function, one whose binding is assigned, one used in any other way, one that reads `arguments`, and any file with `eval(…)` or `with` are skipped. A value is a primitive (also signed and `void 0`), `undefined` (a missing argument too), or a binding the function can already see that is never assigned; spread arguments, ambiguous values and nested destructuring are skipped, and a call inside the function that forwards the parameter itself does not count. Shallow object-destructured properties are checked one by one when every argument is an object literal. Calls inside the function do not count toward `minimumCallCount` (default 2, a function called once is left alone). The fix removes the parameter and its argument from every call and writes the value where the parameter was used, for one parameter of a function at a time. Behaviour of eslint-plugin-unicorn v79.0.0 (read, not copied); no type information is read, so a predicate's parameter is skipped only when the signature says so.

**Why:** A parameter that never varies is a constant in disguise: it makes every caller repeat it and every reader wonder what else it could be.

```codelaw
native: js/no-unnecessary-parameters
```

## Examples

### JavaScript

#### Don't

```js
function log(message, level) {
  console.log(level, message);
}

log('Starting', 'info');
log('Finished', 'info');
```

#### Do

```js
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```

#### Fixed

```js
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```


### JSX

#### Don't

```jsx
function log(message, level) {
  console.log(level, message);
}

log('Starting', 'info');
log('Finished', 'info');
```

#### Do

```jsx
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```

#### Fixed

```jsx
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```


### TypeScript

#### Don't

```ts
function log(message, level) {
  console.log(level, message);
}

log('Starting', 'info');
log('Finished', 'info');
```

#### Do

```ts
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```

#### Fixed

```ts
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```


### TSX

#### Don't

```tsx
function log(message, level) {
  console.log(level, message);
}

log('Starting', 'info');
log('Finished', 'info');
```

#### Do

```tsx
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```

#### Fixed

```tsx
function log(message) {
  console.log('info', message);
}

log('Starting');
log('Finished');
```
