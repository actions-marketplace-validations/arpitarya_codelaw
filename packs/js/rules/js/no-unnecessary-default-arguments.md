---
id: js/no-unnecessary-default-arguments
title: "No argument that equals the parameter's default"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: `auto` in JavaScript; in TypeScript a suggestion, since the type of the call may depend on the argument"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-default-arguments]
since: 0.5.0
---

A call of a local function (a declaration, or a `const` holding a function) whose trailing arguments are primitive literals equal to their parameters' default values (`r(true)` for `function r(a = true)`) is reported at the first of them, and the fix removes them. Arguments in the middle are kept, and nothing is removed after a spread. A function that reads `arguments`, has type parameters, or is reassigned is left alone; so are defaults of destructured object parameters.

**Why:** Passing the default again repeats it in every call and hides when a call really chooses a value.

```codelaw
native: js/no-unnecessary-default-arguments
```

## Examples

### JavaScript

#### Don't

```js
function render(animate = true) {}
render(true);
```

#### Do

```js
function render(animate = true) {}
render();
```

#### Fixed

```js
function render(animate = true) {}
render();
```


### JSX

#### Don't

```jsx
function render(animate = true) {}
render(true);
```

#### Do

```jsx
function render(animate = true) {}
render();
```

#### Fixed

```jsx
function render(animate = true) {}
render();
```


### TypeScript

#### Don't

```ts
function render(animate = true) {}
render(true);
```

#### Do

```ts
function render(animate = true) {}
render();
```

#### Fixed

```ts
function render(animate = true) {}
render();
```


### TSX

#### Don't

```tsx
function render(animate = true) {}
render(true);
```

#### Do

```tsx
function render(animate = true) {}
render();
```

#### Fixed

```tsx
function render(animate = true) {}
render();
```
