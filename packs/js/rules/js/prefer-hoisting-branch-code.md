---
id: js/prefer-hoisting-branch-code
title: "No statements repeated at the start or end of every branch"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "the subset: code repeated at the end of every branch is moved after the `if` automatically; code at the start is moved before it as a suggestion, since the tests then run after it"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-hoisting-branch-code]
since: 0.5.0
---

In a complete `if / else if / else` chain whose branches are blocks, statements that every branch starts with, or ends with, written the same (layout and comments aside), are reported. The fix removes them from each branch and writes them once, after the `if` for trailing code, before it for leading code. A branch the move would empty, a shared declaration, a multi-line string, a comment in a branch, and an `if` that is not in a statement list leave it without a fix.

**Why:** Code repeated in every branch is not conditional; written once it can't drift apart.

```codelaw
native: js/prefer-hoisting-branch-code
```

## Examples

### JavaScript

#### Don't

```js
if (admin) {
  grant();
  log(user);
} else {
  deny();
  log(user);
}
```

#### Do

```js
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```

#### Fixed

```js
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```


### JSX

#### Don't

```jsx
if (admin) {
  grant();
  log(user);
} else {
  deny();
  log(user);
}
```

#### Do

```jsx
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```

#### Fixed

```jsx
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```


### TypeScript

#### Don't

```ts
if (admin) {
  grant();
  log(user);
} else {
  deny();
  log(user);
}
```

#### Do

```ts
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```

#### Fixed

```ts
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```


### TSX

#### Don't

```tsx
if (admin) {
  grant();
  log(user);
} else {
  deny();
  log(user);
}
```

#### Do

```tsx
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```

#### Fixed

```tsx
if (admin) {
  grant();
} else {
  deny();
}
log(user);
```
