---
id: js/stateful-regex
title: "No global regular expression used in a way its `lastIndex` breaks"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether to drop `g`, reset `lastIndex` or create the expression where it is used is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/stateful-regex]
since: 0.5.0
---

Three shapes are reported. A regular expression with both `g` and `y` (literal or `RegExp(…, "gy")`), where `g` does nothing. A `/…/g.exec()` (or `new RegExp(…, "g").exec()`) written in a `while`/`do…while` condition, which makes a new expression each pass so the loop never ends. And a variable bound once to a `g` expression whose `test()`/`exec()` calls are given different inputs, with no `lastIndex` reset anywhere: each call starts where the last one stopped, so a later input is searched from the middle.

**Why:** A global expression remembers where its last match ended; code that forgets this skips matches or loops forever.

```codelaw
native: js/stateful-regex
```

## How to fix

1. Drop the `g` flag when only a yes/no answer is wanted, reset `lastIndex = 0` before each new input, or create the expression once, before the loop.

## Examples

### JavaScript

#### Don't

```js
const word = /a/g;
word.test(first);
word.test(second);
```

#### Do

```js
const word = /a/;
word.test(first);
word.test(second);
```


### JSX

#### Don't

```jsx
const word = /a/g;
word.test(first);
word.test(second);
```

#### Do

```jsx
const word = /a/;
word.test(first);
word.test(second);
```


### TypeScript

#### Don't

```ts
const word = /a/g;
word.test(first);
word.test(second);
```

#### Do

```ts
const word = /a/;
word.test(first);
word.test(second);
```


### TSX

#### Don't

```tsx
const word = /a/g;
word.test(first);
word.test(second);
```

#### Do

```tsx
const word = /a/;
word.test(first);
word.test(second);
```
