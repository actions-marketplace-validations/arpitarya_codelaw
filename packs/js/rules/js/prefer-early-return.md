---
id: js/prefer-early-return
title: "An early `return` instead of a function body wrapped in one `if`"
languages: [javascript, jsx, typescript, tsx]
fix: auto
options: { maximumStatements: { type: "integer", default: 1, description: "report only when the if holds more statements than this" } }
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-early-return]
since: 0.5.0
---

When a function's body ends with an `if` (no `else`) holding more than `maximumStatements` (1) statements, the `if` is reported and inverted: `if (!cond) { return; }` followed by its statements, one level out. The condition is negated as `js/prefer-continue` negates it, and the same shapes leave it without a fix; a comment after the `if` makes the fix a suggestion.

**Why:** A guard that returns early keeps the function's main path at one level of indentation.

```codelaw
native: js/prefer-early-return
```

## Examples

### JavaScript

#### Don't

```js
function save(form) {
  if (form.valid) {
    send(form);
    close(form);
  }
}
```

#### Do

```js
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```

#### Fixed

```js
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```


### JSX

#### Don't

```jsx
function save(form) {
  if (form.valid) {
    send(form);
    close(form);
  }
}
```

#### Do

```jsx
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```

#### Fixed

```jsx
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```


### TypeScript

#### Don't

```ts
function save(form) {
  if (form.valid) {
    send(form);
    close(form);
  }
}
```

#### Do

```ts
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```

#### Fixed

```ts
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```


### TSX

#### Don't

```tsx
function save(form) {
  if (form.valid) {
    send(form);
    close(form);
  }
}
```

#### Do

```tsx
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```

#### Fixed

```tsx
function save(form) {
  if (!form.valid) {
    return;
  }

  send(form);
  close(form);
}
```
