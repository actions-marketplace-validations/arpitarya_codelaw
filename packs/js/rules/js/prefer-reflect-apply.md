---
id: js/prefer-reflect-apply
title: "Reflect.apply() rather than Function#apply with no receiver"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-reflect-apply]
since: 0.5.0
---

A function called with an argument list and no receiver is called with `Reflect.apply(fn, null, args)`, not `fn.apply(null, args)` or `Function.prototype.apply.call(fn, null, args)`.

**Why:** `Reflect.apply` cannot be broken by a function that defines its own `apply` property, and it reads as what it does.

```codelaw
match:
  any:
    - pattern: "$F.apply($T, $ARGS)"
      not: { has: { field: function, has: { field: object, regex: '^(Reflect|Function\.prototype\.apply)$' } } }
    - pattern: "Function.prototype.apply.call($F, $T, $ARGS)"
constraints:
  T: { regex: '^(null|undefined)$' }
message: "Call `Reflect.apply($F, $T, $ARGS)`"
fix: "Reflect.apply($F, $T, $ARGS)"
```

## Examples

### JavaScript

#### Don't

```js
const max = Math.max.apply(null, values);
```

#### Do

```js
const max = Reflect.apply(Math.max, null, values);
```


### JSX

#### Don't

```jsx
const max = Math.max.apply(null, values);
```

#### Do

```jsx
const max = Reflect.apply(Math.max, null, values);
```


### TypeScript

#### Don't

```ts
const max = Math.max.apply(null, values);
```

#### Do

```ts
const max = Reflect.apply(Math.max, null, values);
```


### TSX

#### Don't

```tsx
const max = Math.max.apply(null, values);
```

#### Do

```tsx
const max = Reflect.apply(Math.max, null, values);
```
