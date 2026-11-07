---
id: js/prefer-classlist-toggle
title: "classList.toggle() rather than add or remove by a condition"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-classlist-toggle]
since: 0.5.0
---

A class added when a condition holds and removed otherwise is set with `el.classList.toggle(name, condition)`, not an `if`-`else` of `add` and `remove` or a computed `classList[c ? "add" : "remove"]`.

**Why:** `toggle`'s second argument does exactly this in one call.

```codelaw
match:
  any:
    - pattern: "if ($C) { $E.classList.add($N); } else { $E.classList.remove($N); }"
      all: [{ pattern: $IF }]
    - pattern: "if ($C) $E.classList.add($N); else $E.classList.remove($N);"
      all: [{ pattern: $IF }]
    - pattern: "$E.classList[$C ? $ADD : $REM]($N)"
      all: [{ pattern: $CALL }]
constraints:
  ADD: { regex: '^["'']add["'']$' }
  REM: { regex: '^["'']remove["'']$' }
message: "Use `$E.classList.toggle($N, $C)`"
fix:
  - replace: { target: $IF, with: "$E.classList.toggle($N, $C);" }
  - replace: { target: $CALL, with: "$E.classList.toggle($N, $C)" }
```

## Examples

### JavaScript

#### Don't

```js
if (open) {
  menu.classList.add("is-open");
} else {
  menu.classList.remove("is-open");
}
```

#### Do

```js
menu.classList.toggle("is-open", open);
```


### JSX

#### Don't

```jsx
if (open) {
  menu.classList.add("is-open");
} else {
  menu.classList.remove("is-open");
}
```

#### Do

```jsx
menu.classList.toggle("is-open", open);
```


### TypeScript

#### Don't

```ts
if (open) {
  menu.classList.add("is-open");
} else {
  menu.classList.remove("is-open");
}
```

#### Do

```ts
menu.classList.toggle("is-open", open);
```


### TSX

#### Don't

```tsx
if (open) {
  menu.classList.add("is-open");
} else {
  menu.classList.remove("is-open");
}
```

#### Do

```tsx
menu.classList.toggle("is-open", open);
```
