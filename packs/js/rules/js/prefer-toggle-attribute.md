---
id: js/prefer-toggle-attribute
title: "toggleAttribute() rather than setting or removing by a condition"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-toggle-attribute]
since: 0.5.0
---

An attribute set to the empty string when a condition holds and removed otherwise is set with `el.toggleAttribute(name, condition)`.

**Why:** `toggleAttribute` does both halves in one call, and its second argument is the condition.

```codelaw
match:
  any:
    - pattern: "if ($C) { $E.setAttribute($N, $V); } else { $E.removeAttribute($N); }"
    - pattern: "if ($C) $E.setAttribute($N, $V); else $E.removeAttribute($N);"
constraints:
  V: { regex: '^(""|''''|``)$' }
message: "Use `$E.toggleAttribute($N, $C)`"
fix: "$E.toggleAttribute($N, $C);"
```

## Examples

### JavaScript

#### Don't

```js
if (busy) {
  button.setAttribute("disabled", "");
} else {
  button.removeAttribute("disabled");
}
```

#### Do

```js
button.toggleAttribute("disabled", busy);
```


### JSX

#### Don't

```jsx
if (busy) {
  button.setAttribute("disabled", "");
} else {
  button.removeAttribute("disabled");
}
```

#### Do

```jsx
button.toggleAttribute("disabled", busy);
```


### TypeScript

#### Don't

```ts
if (busy) {
  button.setAttribute("disabled", "");
} else {
  button.removeAttribute("disabled");
}
```

#### Do

```ts
button.toggleAttribute("disabled", busy);
```


### TSX

#### Don't

```tsx
if (busy) {
  button.setAttribute("disabled", "");
} else {
  button.removeAttribute("disabled");
}
```

#### Do

```tsx
button.toggleAttribute("disabled", busy);
```
