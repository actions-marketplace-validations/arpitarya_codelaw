---
id: js/prefer-object-iterable-methods
title: "The `Object` iterable method that matches what is used"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the loop or callback is rewritten when its body reads the object only as `object[key]`, does not write to it, has no jump out of the loop and no call before or after a read, and no comment is inside; otherwise it is reported without a fix"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-object-iterable-methods]
since: 0.5.0
---

Reported at the `Object` method name: a `for…of` over `Object.keys(object)`, or a `.map()` callback over it, whose body reads `object[key]` (`Object.values(object)` when the key is used for nothing else, `Object.entries(object)` when it is); and a `for…of` or a callback (`map`, `every`, `some`, `findIndex`, `findLastIndex`, `flatMap`, `forEach`, `reduce`, `reduceRight`) over `Object.entries(object)` that uses only the key (`Object.keys`) or only the value (`Object.values`) of its `[key, value]`. The fix moves the method, the binding and each `object[key]` together. `Object.values()` and `Object.entries()` read every value before the first iteration, while `object[key]` reads it during, so a body that writes to the object, calls something between reads, or leaves the loop early is reported without a fix; so is one with a TypeScript cast on the object (upstream moves the cast, this rule does not). A key that is written, destructured or annotated, or an inline callback that is async, a generator, or uses `arguments`, is not reported.

**Why:** Reading `object[key]` for each key looks every value up again, and a pair of which one half is dropped says more than it uses.

```codelaw
native: js/prefer-object-iterable-methods
```

## Examples

### JavaScript

#### Don't

```js
for (const key of Object.keys(object)) {
  foo(object[key]);
}
```

#### Do

```js
for (const value of Object.values(object)) {
  foo(value);
}
```

#### Fixed

```js
for (const value of Object.values(object)) {
  foo(value);
}
```


### JSX

#### Don't

```jsx
for (const key of Object.keys(object)) {
  foo(object[key]);
}
```

#### Do

```jsx
for (const value of Object.values(object)) {
  foo(value);
}
```

#### Fixed

```jsx
for (const value of Object.values(object)) {
  foo(value);
}
```


### TypeScript

#### Don't

```ts
for (const key of Object.keys(object)) {
  foo(object[key]);
}
```

#### Do

```ts
for (const value of Object.values(object)) {
  foo(value);
}
```

#### Fixed

```ts
for (const value of Object.values(object)) {
  foo(value);
}
```


### TSX

#### Don't

```tsx
for (const key of Object.keys(object)) {
  foo(object[key]);
}
```

#### Do

```tsx
for (const value of Object.values(object)) {
  foo(value);
}
```

#### Fixed

```tsx
for (const value of Object.values(object)) {
  foo(value);
}
```
