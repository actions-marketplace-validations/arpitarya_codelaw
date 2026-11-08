---
id: js/no-immediate-mutation
title: "No changing a new literal in the very next statement"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: merging is `auto` when the added values have no side effects; with side effects (or in TypeScript, for `let x; x = v`) it is a suggestion"
stability: preview
ported_from: [eslint-plugin-unicorn/no-immediate-mutation]
since: 0.5.0
---

A variable given an array, object, `Set` or `Map` literal and changed by the next statement — `a.push(…)`/`a.unshift(…)`, `o.key = v`, `Object.assign(o, src)`, `s.add(v)`, `m.set(k, v)` — is reported at the change, which belongs in the literal. So is `let x;` followed by `x = v;`. The fix merges the change into the literal (or the declaration) and removes the statement; a value that calls something makes it a suggestion, and an accessor, a `__proto__` key, an `unshift` before side-effecting items or a comment leaves it without one. A change that reads the variable itself is not reported.

**Why:** Building the value in one expression shows its whole content where it is created.

```codelaw
native: js/no-immediate-mutation
```

## Examples

### JavaScript

#### Don't

```js
const ids = [1];
ids.push(2);
```

#### Do

```js
const ids = [1, 2];
```

#### Fixed

```js
const ids = [1, 2];
```


### JSX

#### Don't

```jsx
const ids = [1];
ids.push(2);
```

#### Do

```jsx
const ids = [1, 2];
```

#### Fixed

```jsx
const ids = [1, 2];
```


### TypeScript

#### Don't

```ts
const ids = [1];
ids.push(2);
```

#### Do

```ts
const ids = [1, 2];
```

#### Fixed

```ts
const ids = [1, 2];
```


### TSX

#### Don't

```tsx
const ids = [1];
ids.push(2);
```

#### Do

```tsx
const ids = [1, 2];
```

#### Fixed

```tsx
const ids = [1, 2];
```
